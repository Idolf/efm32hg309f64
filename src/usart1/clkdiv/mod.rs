#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKDIV {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
#[inline]     pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
#[inline]     pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
#[inline]     pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
#[inline]     pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct DIVEXTR {
    bits: u8,
}
impl DIVEXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
#[inline]     pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIVR {
    bits: u16,
}
impl DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
#[inline]     pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DIVEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVEXTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
#[inline]     pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
#[inline]     pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 32767;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
#[inline]     pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 3:5 - Extended Fractional Clock Divider"]
    #[inline]
#[inline]     pub fn divext(&self) -> DIVEXTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVEXTR { bits }
    }
    #[doc = "Bits 6:20 - Fractional Clock Divider"]
    #[inline]
#[inline]     pub fn div(&self) -> DIVR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DIVR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
#[inline]     pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
#[inline]     pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:5 - Extended Fractional Clock Divider"]
    #[inline]
#[inline]     pub fn divext(&mut self) -> _DIVEXTW {
        _DIVEXTW { w: self }
    }
    #[doc = "Bits 6:20 - Fractional Clock Divider"]
    #[inline]
#[inline]     pub fn div(&mut self) -> _DIVW {
        _DIVW { w: self }
    }
}
