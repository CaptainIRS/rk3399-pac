#[doc = "Register `SARADC_STAS` reader"]
pub type R = crate::R<SaradcStasSpec>;
#[doc = "ADC status (EOC)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcStatus {
    #[doc = "0: ADC stop"]
    B0 = 0,
    #[doc = "1: Conversion in progress"]
    B1 = 1,
}
impl From<AdcStatus> for bool {
    #[inline(always)]
    fn from(variant: AdcStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_STATUS` reader - ADC status (EOC)"]
pub type AdcStatusR = crate::BitReader<AdcStatus>;
impl AdcStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcStatus {
        match self.bits {
            false => AdcStatus::B0,
            true => AdcStatus::B1,
        }
    }
    #[doc = "ADC stop"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AdcStatus::B0
    }
    #[doc = "Conversion in progress"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AdcStatus::B1
    }
}
impl R {
    #[doc = "Bit 0 - ADC status (EOC)"]
    #[inline(always)]
    pub fn adc_status(&self) -> AdcStatusR {
        AdcStatusR::new((self.bits & 1) != 0)
    }
}
#[doc = "The status register of A/D Converter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_stas::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaradcStasSpec;
impl crate::RegisterSpec for SaradcStasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saradc_stas::R`](R) reader structure"]
impl crate::Readable for SaradcStasSpec {}
#[doc = "`reset()` method sets SARADC_STAS to value 0"]
impl crate::Resettable for SaradcStasSpec {
    const RESET_VALUE: u32 = 0;
}
