#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMD {
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
}
#[doc = r" Proxy"]
pub struct _EM4WUCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _EM4WUCLRW<'a> {
    #[doc = r" Sets the field bit"]
#[inline]     pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
#[inline]     pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
#[inline]     pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - EM4 Wake-up clear"]
    #[inline]
#[inline]     pub fn em4wuclr(&mut self) -> _EM4WUCLRW {
        _EM4WUCLRW { w: self }
    }
}
