#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXDATAX {
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
pub struct _TXDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
#[inline]     pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXBREAKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBREAKW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXDISATW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDISATW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXENATW<'a> {
    w: &'a mut W,
}
impl<'a> _RXENATW<'a> {
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:8 - TX Data"]
    #[inline]
#[inline]     pub fn txdata(&mut self) -> _TXDATAW {
        _TXDATAW { w: self }
    }
    #[doc = "Bit 13 - Transmit Data As Break"]
    #[inline]
#[inline]     pub fn txbreak(&mut self) -> _TXBREAKW {
        _TXBREAKW { w: self }
    }
    #[doc = "Bit 14 - Disable TX After Transmission"]
    #[inline]
#[inline]     pub fn txdisat(&mut self) -> _TXDISATW {
        _TXDISATW { w: self }
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline]
#[inline]     pub fn rxenat(&mut self) -> _RXENATW {
        _RXENATW { w: self }
    }
}
