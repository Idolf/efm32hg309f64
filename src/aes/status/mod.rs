#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
#[inline]     pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RUNNINGR {
    bits: bool,
}
impl RUNNINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
#[inline]     pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
#[inline]     pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
#[inline]     pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
#[inline]     pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - AES Running"]
    #[inline]
#[inline]     pub fn running(&self) -> RUNNINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNNINGR { bits }
    }
}
