use std::vec;

trait Cake {
  fn price(&self) -> usize;
  fn name(&self) -> String {
    self
      .name_accumulator()
      .into_iter()
      .fold((0, String::new()), |(position, mut s), n| {
        match position {
          0 => s.push_str(&n),
          1 => s.push_str(&format!(" with {n}")),
          _ => s.push_str(&format!(" and {n}")),
        }
        (position + 1, s)
      })
      .1
  }
  fn name_accumulator(&self) -> Vec<String> {
    vec![self.name_constant().into()]
  }
  fn name_constant(&self) -> &'static str;
}

struct Cookie;
impl Cake for Cookie {
  fn price(&self) -> usize {
    200
  }

  fn name_constant(&self) -> &'static str {
    "🍪"
  }
}

struct Cupcake;
impl Cake for Cupcake {
  fn price(&self) -> usize {
    100
  }

  fn name_constant(&self) -> &'static str {
    "🧁"
  }
}

struct Chocolate<C: Cake>(C);
impl<C: Cake> Cake for Chocolate<C> {
  fn price(&self) -> usize {
    &self.0.price() + 10
  }

  fn name_constant(&self) -> &'static str {
    "🍫"
  }

  fn name_accumulator(&self) -> Vec<String> {
    let mut vec = self.0.name_accumulator();
    vec.push(self.name_constant().into());
    vec
  }
}

struct Nuts<C: Cake>(C);
impl<C: Cake> Cake for Nuts<C> {
  fn price(&self) -> usize {
    &self.0.price() + 20
  }

  fn name_constant(&self) -> &'static str {
    "🥜"
  }

  fn name_accumulator(&self) -> Vec<String> {
    let mut vec = self.0.name_accumulator();
    vec.push(self.name_constant().into());
    vec
  }
}

#[derive(Default)]
struct Bundle {
  inners: Vec<Box<dyn Cake>>,
  inners_bundle: Vec<Box<dyn Bundle>>,
}

impl Bundle {
  pub fn add(&mut self, cake: impl Cake + 'static) {
    self.inners.push(Box::new(cake))
  }

  pub fn add(&mut self, b: impl Bundle + 'static) {
    self.inners_bundle.push(Box::new(b))
  }

  pub fn price(&self) -> usize {
    self.inners.iter().map(|c| c.price()).sum::<usize>() * 9 / 10
  }
}

fn main() {}
// tests module
#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  #[test]
  fn test_bundle() {
    let mut bundle = Bundle::default();
    bundle.add(Cookie);
    assert_eq!(bundle.price(), 180)
  }

  #[test]
  fn test_bundle_cookie_cupcake() {
    let mut bundle = Bundle::default();
    bundle.add(Cookie);
    bundle.add(Cupcake);
    assert_eq!(bundle.price(), 180 + 90)
  }

  #[test]
  fn test_bundle_of_bundle() {

    let mut subbundle = Bundle::default();
    subbundle.add(Cookie);
    subbundle.add(Cupcake);

    let mut bundle = Bundle::default();
    bundle.add(Cookie);
    bundle.add(subbundle);

    assert_eq!(bundle.price(), 180 + 90)
  }
  #[test]
  fn test_cookie_price() {
    assert_eq!(Cookie.price(), 200)
  }

  #[test]
  fn test_cookie_name() {
    assert_eq!(Cookie.name(), "🍪")
  }

  #[test]
  fn test_cupcake() {
    assert_eq!(Cupcake.price(), 100);
    assert_eq!(Cupcake.name(), "🧁");
  }

  #[test]
  fn test_cupcake_with_chocolate() {
    let sut = Chocolate(Cupcake);
    assert_eq!(sut.price(), 110);
    assert_eq!(sut.name(), "🧁 with 🍫");
  }
  #[test]
  fn test_cookie_with_chocolate() {
    let sut = Chocolate(Cookie);
    assert_eq!(sut.price(), 210);
    assert_eq!(sut.name(), "🍪 with 🍫");
  }
  #[test]
  fn test_cookie_with_nuts() {
    let sut = Nuts(Cookie);
    assert_eq!(sut.price(), 220);
    assert_eq!(sut.name(), "🍪 with 🥜");
  }

  #[test]
  fn test_cookie_with_nuts_and_chocolate() {
    let sut = Chocolate(Nuts(Cookie));
    assert_eq!(sut.price(), 230);
    assert_eq!(sut.name(), "🍪 with 🥜 and 🍫");
  }

  #[test]
  fn test_cookie_with_chocolate_and_nuts() {
    let sut = Nuts(Chocolate(Cookie));
    assert_eq!(sut.price(), 230);
    assert_eq!(sut.name(), "🍪 with 🍫 and 🥜");
  }
}
