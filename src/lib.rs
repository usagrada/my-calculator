#![feature(asm)]
#![feature(global_asm)]

pub mod ast;
pub mod cogen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
