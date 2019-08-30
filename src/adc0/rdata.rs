#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RDATA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ADC1RDTRR {
    bits: u16,
}
impl ADC1RDTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RDATAR {
    bits: u16,
}
impl RDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:31 - ADC regular channel data"]
    #[inline]
    pub fn adc1rdtr(&self) -> ADC1RDTRR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ADC1RDTRR { bits }
    }
    #[doc = "Bits 0:15 - Regular channel data"]
    #[inline]
    pub fn rdata(&self) -> RDATAR {
        let bits = {
            const MASK: u16 = 0xffff;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RDATAR { bits }
    }
}