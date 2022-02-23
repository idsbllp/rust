#[macro_use]
pub mod front_of_house {
  pub mod hosting {
    pub fn add_to_wait_list() {
      println!("add to wait list");
    }

    #[macro_export]
    macro_rules! add_list {
      () => {
        // $crate::ma
      };
    }
  }

  mod serving {
    #[allow(dead_code)]
    fn take_order() {

    }
  }
}

crate::add_list!();

