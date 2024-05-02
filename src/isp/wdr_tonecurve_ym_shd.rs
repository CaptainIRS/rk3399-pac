#[doc = "Register `WDR_TONECURVE_YM_SHD%s` reader"]
pub type R = crate::R<WdrTonecurveYmShdSpec>;
#[doc = "Field `tonecurve_ym_n_shd` reader - Tone curve value definition y-axis (output) of\n\nWDR unit shadow register."]
pub type TonecurveYmNShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Tone curve value definition y-axis (output) of\n\nWDR unit shadow register."]
    #[inline(always)]
    pub fn tonecurve_ym_n_shd(&self) -> TonecurveYmNShdR {
        TonecurveYmNShdR::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "Tonemapping curve coefficient shadow register n (n=0..32)\n\nNote: The reset values define a linear curve which has the same effect as bypass. Reset \n\nvalues are: Ym_00 = 0x0000, Ym_01 = 0x0080, Ym_02 = 0x0100, Ym_03 = 0x0180, Ym_04 \n\n= 0x0200, \n\n\n\nYm_05 = 0x0280, Ym_06 = 0x0300, Ym_07 = 0x0380, Ym_08 = 0x0400, Ym_09 = \n\n0x0480, Ym_10 = 0x0500, Ym_11 = 0x0580, Ym_12 = 0x0600, Ym_13 = 0x0680, Ym_14 = \n\n0x0700, Ym_15 = 0x0780, Ym_16 = 0x0800, Ym_17 = 0x0880, Ym_18 = 0x0900, Ym_19 = \n\n0x0980, Ym_20 = 0x0A00, Ym_21 = 0x0A80, Ym_22 = 0x0B00, Ym_23 = 0x0B80, Ym_24 = \n\n0x0C00, Ym_25 = 0x0C80, Ym_26 = 0x0D00, Ym_27 = 0x0D80, Ym_28 = 0x0E00, Ym_29 = \n\n0x0E80, Ym_30 = 0x0F00, Ym_31 = 0x0F80, Ym_32 = 0x1000 \n\n\n\n \n\nData format: 13 bit unsigned \n\n \n\n\n\nRESTRICTION: each Y must be in the +2047/-2048 range compared to its predecessor (so \n\nthat the difference between successive Y values is 12-bit signed !) \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_tonecurve_ym_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrTonecurveYmShdSpec;
impl crate::RegisterSpec for WdrTonecurveYmShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_tonecurve_ym_shd::R`](R) reader structure"]
impl crate::Readable for WdrTonecurveYmShdSpec {}
#[doc = "`reset()` method sets WDR_TONECURVE_YM_SHD%s to value 0"]
impl crate::Resettable for WdrTonecurveYmShdSpec {
    const RESET_VALUE: u32 = 0;
}
