#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct DECRYPTR {
    bits: bool,
}
impl DECRYPTR {
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
#[doc = r" Value of the field"]
pub struct DATASTARTR {
    bits: bool,
}
impl DATASTARTR {
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
#[doc = r" Value of the field"]
pub struct XORSTARTR {
    bits: bool,
}
impl XORSTARTR {
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
#[doc = r" Value of the field"]
pub struct BYTEORDERR {
    bits: bool,
}
impl BYTEORDERR {
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
#[doc = r" Proxy"]
pub struct _DECRYPTW<'a> {
    w: &'a mut W,
}
impl<'a> _DECRYPTW<'a> {
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
#[doc = r" Proxy"]
pub struct _DATASTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _DATASTARTW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XORSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _XORSTARTW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYTEORDERW<'a> {
    w: &'a mut W,
}
impl<'a> _BYTEORDERW<'a> {
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
    #[doc = "Bit 0 - Decryption/Encryption Mode"]
    #[inline]
#[inline]     pub fn decrypt(&self) -> DECRYPTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DECRYPTR { bits }
    }
    #[doc = "Bit 4 - AES_DATA Write Start"]
    #[inline]
#[inline]     pub fn datastart(&self) -> DATASTARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATASTARTR { bits }
    }
    #[doc = "Bit 5 - AES_XORDATA Write Start"]
    #[inline]
#[inline]     pub fn xorstart(&self) -> XORSTARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XORSTARTR { bits }
    }
    #[doc = "Bit 6 - Configure byte order in data and key registers"]
    #[inline]
#[inline]     pub fn byteorder(&self) -> BYTEORDERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYTEORDERR { bits }
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
    #[doc = "Bit 0 - Decryption/Encryption Mode"]
    #[inline]
#[inline]     pub fn decrypt(&mut self) -> _DECRYPTW {
        _DECRYPTW { w: self }
    }
    #[doc = "Bit 4 - AES_DATA Write Start"]
    #[inline]
#[inline]     pub fn datastart(&mut self) -> _DATASTARTW {
        _DATASTARTW { w: self }
    }
    #[doc = "Bit 5 - AES_XORDATA Write Start"]
    #[inline]
#[inline]     pub fn xorstart(&mut self) -> _XORSTARTW {
        _XORSTARTW { w: self }
    }
    #[doc = "Bit 6 - Configure byte order in data and key registers"]
    #[inline]
#[inline]     pub fn byteorder(&mut self) -> _BYTEORDERW {
        _BYTEORDERW { w: self }
    }
}
