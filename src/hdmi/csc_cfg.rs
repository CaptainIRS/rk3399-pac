#[doc = "Register `CSC_CFG` reader"]
pub type R = crate::R<CscCfgSpec>;
#[doc = "Register `CSC_CFG` writer"]
pub type W = crate::W<CscCfgSpec>;
#[doc = "Field `DECMODE` reader - Chroma decimation configuration:\n\n decmode\\[1:0\\]
| Chroma Decimation\n\n 00 | decimation disabled\n\n 01 | Hd (z) =1\n\n 10 | Hd(z)=1/ 4 + 1/2z^(-1 )+1/4 z^(-2)\n\n 11 | Hd(z)x2^(11)= -5+12z^(-2) - 22z^(-\n\n4)+39z^(-8)\n\n +109z^(-10) -204z^(-12)+648z^(-14) +\n\n1024z^(-15) +648z^(-16) -\n\n 204z^(-18) +109z^(-20)- 65z^(-22) +39z^(-24) -\n\n22z^(-26) +12z^(-\n\n 28)-5z^(-30)"]
pub type DecmodeR = crate::FieldReader;
#[doc = "Field `DECMODE` writer - Chroma decimation configuration:\n\n decmode\\[1:0\\]
| Chroma Decimation\n\n 00 | decimation disabled\n\n 01 | Hd (z) =1\n\n 10 | Hd(z)=1/ 4 + 1/2z^(-1 )+1/4 z^(-2)\n\n 11 | Hd(z)x2^(11)= -5+12z^(-2) - 22z^(-\n\n4)+39z^(-8)\n\n +109z^(-10) -204z^(-12)+648z^(-14) +\n\n1024z^(-15) +648z^(-16) -\n\n 204z^(-18) +109z^(-20)- 65z^(-22) +39z^(-24) -\n\n22z^(-26) +12z^(-\n\n 28)-5z^(-30)"]
pub type DecmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPARE_1` reader - Reserved as \"spare\" register with no associated\n\nfunctionality."]
pub type Spare1R = crate::FieldReader;
#[doc = "Field `SPARE_1` writer - Reserved as \"spare\" register with no associated\n\nfunctionality."]
pub type Spare1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTMODE` reader - Chroma interpolation configuration:\n\nintmode\\[1:0\\]
| Chroma Interpolation\n\n00 | interpolation disabled\n\n01 | Hu (z) =1 + z^(-1)\n\n10 | Hu(z)=1/ 2 + z^(-11)+1/2 z^(-2)\n\n11 | interpolation disabled"]
pub type IntmodeR = crate::FieldReader;
#[doc = "Field `INTMODE` writer - Chroma interpolation configuration:\n\nintmode\\[1:0\\]
| Chroma Interpolation\n\n00 | interpolation disabled\n\n01 | Hu (z) =1 + z^(-1)\n\n10 | Hu(z)=1/ 2 + z^(-11)+1/2 z^(-2)\n\n11 | interpolation disabled"]
pub type IntmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPARE_2` reader - Reserved as \"spare\" register with no associated\n\nfunctionality."]
pub type Spare2R = crate::BitReader;
#[doc = "Field `SPARE_2` writer - Reserved as \"spare\" register with no associated\n\nfunctionality."]
pub type Spare2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC_LIMIT` reader - When set (1'b1), the range limitation values\n\ndefined in registers csc_mat_uplim and\n\ncsc_mat_dnlim are applied to the output of the\n\nColor Space Conversion matrix. This feature\n\nensures that the video output range is always\n\nrespected, independently of the matrix coefficient\n\nconfiguration or of the video input stream."]
pub type CscLimitR = crate::BitReader;
#[doc = "Field `CSC_LIMIT` writer - When set (1'b1), the range limitation values\n\ndefined in registers csc_mat_uplim and\n\ncsc_mat_dnlim are applied to the output of the\n\nColor Space Conversion matrix. This feature\n\nensures that the video output range is always\n\nrespected, independently of the matrix coefficient\n\nconfiguration or of the video input stream."]
pub type CscLimitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Chroma decimation configuration:\n\n decmode\\[1:0\\]
| Chroma Decimation\n\n 00 | decimation disabled\n\n 01 | Hd (z) =1\n\n 10 | Hd(z)=1/ 4 + 1/2z^(-1 )+1/4 z^(-2)\n\n 11 | Hd(z)x2^(11)= -5+12z^(-2) - 22z^(-\n\n4)+39z^(-8)\n\n +109z^(-10) -204z^(-12)+648z^(-14) +\n\n1024z^(-15) +648z^(-16) -\n\n 204z^(-18) +109z^(-20)- 65z^(-22) +39z^(-24) -\n\n22z^(-26) +12z^(-\n\n 28)-5z^(-30)"]
    #[inline(always)]
    pub fn decmode(&self) -> DecmodeR {
        DecmodeR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Reserved as \"spare\" register with no associated\n\nfunctionality."]
    #[inline(always)]
    pub fn spare_1(&self) -> Spare1R {
        Spare1R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Chroma interpolation configuration:\n\nintmode\\[1:0\\]
| Chroma Interpolation\n\n00 | interpolation disabled\n\n01 | Hu (z) =1 + z^(-1)\n\n10 | Hu(z)=1/ 2 + z^(-11)+1/2 z^(-2)\n\n11 | interpolation disabled"]
    #[inline(always)]
    pub fn intmode(&self) -> IntmodeR {
        IntmodeR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Reserved as \"spare\" register with no associated\n\nfunctionality."]
    #[inline(always)]
    pub fn spare_2(&self) -> Spare2R {
        Spare2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set (1'b1), the range limitation values\n\ndefined in registers csc_mat_uplim and\n\ncsc_mat_dnlim are applied to the output of the\n\nColor Space Conversion matrix. This feature\n\nensures that the video output range is always\n\nrespected, independently of the matrix coefficient\n\nconfiguration or of the video input stream."]
    #[inline(always)]
    pub fn csc_limit(&self) -> CscLimitR {
        CscLimitR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Chroma decimation configuration:\n\n decmode\\[1:0\\]
| Chroma Decimation\n\n 00 | decimation disabled\n\n 01 | Hd (z) =1\n\n 10 | Hd(z)=1/ 4 + 1/2z^(-1 )+1/4 z^(-2)\n\n 11 | Hd(z)x2^(11)= -5+12z^(-2) - 22z^(-\n\n4)+39z^(-8)\n\n +109z^(-10) -204z^(-12)+648z^(-14) +\n\n1024z^(-15) +648z^(-16) -\n\n 204z^(-18) +109z^(-20)- 65z^(-22) +39z^(-24) -\n\n22z^(-26) +12z^(-\n\n 28)-5z^(-30)"]
    #[inline(always)]
    #[must_use]
    pub fn decmode(&mut self) -> DecmodeW<CscCfgSpec> {
        DecmodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Reserved as \"spare\" register with no associated\n\nfunctionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_1(&mut self) -> Spare1W<CscCfgSpec> {
        Spare1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Chroma interpolation configuration:\n\nintmode\\[1:0\\]
| Chroma Interpolation\n\n00 | interpolation disabled\n\n01 | Hu (z) =1 + z^(-1)\n\n10 | Hu(z)=1/ 2 + z^(-11)+1/2 z^(-2)\n\n11 | interpolation disabled"]
    #[inline(always)]
    #[must_use]
    pub fn intmode(&mut self) -> IntmodeW<CscCfgSpec> {
        IntmodeW::new(self, 4)
    }
    #[doc = "Bit 6 - Reserved as \"spare\" register with no associated\n\nfunctionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_2(&mut self) -> Spare2W<CscCfgSpec> {
        Spare2W::new(self, 6)
    }
    #[doc = "Bit 7 - When set (1'b1), the range limitation values\n\ndefined in registers csc_mat_uplim and\n\ncsc_mat_dnlim are applied to the output of the\n\nColor Space Conversion matrix. This feature\n\nensures that the video output range is always\n\nrespected, independently of the matrix coefficient\n\nconfiguration or of the video input stream."]
    #[inline(always)]
    #[must_use]
    pub fn csc_limit(&mut self) -> CscLimitW<CscCfgSpec> {
        CscLimitW::new(self, 7)
    }
}
#[doc = "Color Space Converter Interpolation and Decimation Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCfgSpec;
impl crate::RegisterSpec for CscCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_cfg::R`](R) reader structure"]
impl crate::Readable for CscCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_cfg::W`](W) writer structure"]
impl crate::Writable for CscCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_CFG to value 0"]
impl crate::Resettable for CscCfgSpec {
    const RESET_VALUE: u8 = 0;
}
