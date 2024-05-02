#[doc = "Register `IMG_EFF_TINT` reader"]
pub type R = crate::R<ImgEffTintSpec>;
#[doc = "Register `IMG_EFF_TINT` writer"]
pub type W = crate::W<ImgEffTintSpec>;
#[doc = "Field `incr_cb` reader - Cb increment value of a tint. 7 bits for value, 1 sign bit.\n\nDefault tint is R=162 G=138 B=101, which is used\n\nfor the sepia effect. See below for the calculation of the\n\nentries."]
pub type IncrCbR = crate::FieldReader;
#[doc = "Field `incr_cb` writer - Cb increment value of a tint. 7 bits for value, 1 sign bit.\n\nDefault tint is R=162 G=138 B=101, which is used\n\nfor the sepia effect. See below for the calculation of the\n\nentries."]
pub type IncrCbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `incr_cr` reader - Cr increment value of a tint. 7 bits for value, 1 sign bit.\n\nDefault tint is R=162 G=138 B=101, which is used\n\nfor the sepia effect. See below for the calculation of the\n\nentries."]
pub type IncrCrR = crate::FieldReader;
#[doc = "Field `incr_cr` writer - Cr increment value of a tint. 7 bits for value, 1 sign bit.\n\nDefault tint is R=162 G=138 B=101, which is used\n\nfor the sepia effect. See below for the calculation of the\n\nentries."]
pub type IncrCrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Cb increment value of a tint. 7 bits for value, 1 sign bit.\n\nDefault tint is R=162 G=138 B=101, which is used\n\nfor the sepia effect. See below for the calculation of the\n\nentries."]
    #[inline(always)]
    pub fn incr_cb(&self) -> IncrCbR {
        IncrCbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Cr increment value of a tint. 7 bits for value, 1 sign bit.\n\nDefault tint is R=162 G=138 B=101, which is used\n\nfor the sepia effect. See below for the calculation of the\n\nentries."]
    #[inline(always)]
    pub fn incr_cr(&self) -> IncrCrR {
        IncrCrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cb increment value of a tint. 7 bits for value, 1 sign bit.\n\nDefault tint is R=162 G=138 B=101, which is used\n\nfor the sepia effect. See below for the calculation of the\n\nentries."]
    #[inline(always)]
    #[must_use]
    pub fn incr_cb(&mut self) -> IncrCbW<ImgEffTintSpec> {
        IncrCbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Cr increment value of a tint. 7 bits for value, 1 sign bit.\n\nDefault tint is R=162 G=138 B=101, which is used\n\nfor the sepia effect. See below for the calculation of the\n\nentries."]
    #[inline(always)]
    #[must_use]
    pub fn incr_cr(&mut self) -> IncrCrW<ImgEffTintSpec> {
        IncrCrW::new(self, 8)
    }
}
#[doc = "Chrominance increment values of a tint (used for sepia effect)\n\nNote: Calculation process of incr_cr and incr_cb: \n\ntint values given in RGB format: R G B \n\nconverted to Cb and Cr: Cb = -0.148*R - 0.291*G + 0.439*B + 128 Cr = 0.439*R - \n\n\n\n0.368*G - 0.071*B + 128 \n\ncalculating of the increments inc_Cb = (128 – Cb)/110 inc_Cr = (128 – Cr)/110 \n\nregister entry of the increments with an accuracy of 1/64 incr_cb = inc_Cb * 64 \n\nincr_cr = inc_Cr * 64 \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_tint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_tint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffTintSpec;
impl crate::RegisterSpec for ImgEffTintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_tint::R`](R) reader structure"]
impl crate::Readable for ImgEffTintSpec {}
#[doc = "`write(|w| ..)` method takes [`img_eff_tint::W`](W) writer structure"]
impl crate::Writable for ImgEffTintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMG_EFF_TINT to value 0x880c"]
impl crate::Resettable for ImgEffTintSpec {
    const RESET_VALUE: u32 = 0x880c;
}
