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
pub struct RXENSR {
    bits: bool,
}
impl RXENSR {
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
pub struct TXENSR {
    bits: bool,
}
impl TXENSR {
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
pub struct MASTERR {
    bits: bool,
}
impl MASTERR {
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
pub struct RXBLOCKR {
    bits: bool,
}
impl RXBLOCKR {
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
pub struct TXTRIR {
    bits: bool,
}
impl TXTRIR {
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
pub struct TXCR {
    bits: bool,
}
impl TXCR {
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
pub struct TXBLR {
    bits: bool,
}
impl TXBLR {
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
pub struct RXDATAVR {
    bits: bool,
}
impl RXDATAVR {
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
pub struct RXFULLR {
    bits: bool,
}
impl RXFULLR {
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
pub struct TXBDRIGHTR {
    bits: bool,
}
impl TXBDRIGHTR {
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
pub struct TXBSRIGHTR {
    bits: bool,
}
impl TXBSRIGHTR {
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
pub struct RXDATAVRIGHTR {
    bits: bool,
}
impl RXDATAVRIGHTR {
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
pub struct RXFULLRIGHTR {
    bits: bool,
}
impl RXFULLRIGHTR {
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
    #[doc = "Bit 0 - Receiver Enable Status"]
    #[inline]
#[inline]     pub fn rxens(&self) -> RXENSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXENSR { bits }
    }
    #[doc = "Bit 1 - Transmitter Enable Status"]
    #[inline]
#[inline]     pub fn txens(&self) -> TXENSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXENSR { bits }
    }
    #[doc = "Bit 2 - SPI Master Mode"]
    #[inline]
#[inline]     pub fn master(&self) -> MASTERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MASTERR { bits }
    }
    #[doc = "Bit 3 - Block Incoming Data"]
    #[inline]
#[inline]     pub fn rxblock(&self) -> RXBLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXBLOCKR { bits }
    }
    #[doc = "Bit 4 - Transmitter Tristated"]
    #[inline]
#[inline]     pub fn txtri(&self) -> TXTRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXTRIR { bits }
    }
    #[doc = "Bit 5 - TX Complete"]
    #[inline]
#[inline]     pub fn txc(&self) -> TXCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXCR { bits }
    }
    #[doc = "Bit 6 - TX Buffer Level"]
    #[inline]
#[inline]     pub fn txbl(&self) -> TXBLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXBLR { bits }
    }
    #[doc = "Bit 7 - RX Data Valid"]
    #[inline]
#[inline]     pub fn rxdatav(&self) -> RXDATAVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXDATAVR { bits }
    }
    #[doc = "Bit 8 - RX FIFO Full"]
    #[inline]
#[inline]     pub fn rxfull(&self) -> RXFULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFULLR { bits }
    }
    #[doc = "Bit 9 - TX Buffer Expects Double Right Data"]
    #[inline]
#[inline]     pub fn txbdright(&self) -> TXBDRIGHTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXBDRIGHTR { bits }
    }
    #[doc = "Bit 10 - TX Buffer Expects Single Right Data"]
    #[inline]
#[inline]     pub fn txbsright(&self) -> TXBSRIGHTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXBSRIGHTR { bits }
    }
    #[doc = "Bit 11 - RX Data Right"]
    #[inline]
#[inline]     pub fn rxdatavright(&self) -> RXDATAVRIGHTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXDATAVRIGHTR { bits }
    }
    #[doc = "Bit 12 - RX Full of Right Data"]
    #[inline]
#[inline]     pub fn rxfullright(&self) -> RXFULLRIGHTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXFULLRIGHTR { bits }
    }
}
