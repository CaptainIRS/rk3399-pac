#[doc = "Register `SRSZ_SCALE_LUT` reader"]
pub type R = crate::R<SrszScaleLutSpec>;
#[doc = "Register `SRSZ_SCALE_LUT` writer"]
pub type W = crate::W<SrszScaleLutSpec>;
#[doc = "Field `scale_lut` reader - Entry of lookup table at position scale_lut_addr.\n\nThe lookup table must be filled with appropriate\n\nvalues before the up- scaling functionality can be\n\nused.\n\n"]
pub type ScaleLutR = crate::FieldReader;
#[doc = "Field `scale_lut` writer - Entry of lookup table at position scale_lut_addr.\n\nThe lookup table must be filled with appropriate\n\nvalues before the up- scaling functionality can be\n\nused.\n\n"]
pub type ScaleLutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Entry of lookup table at position scale_lut_addr.\n\nThe lookup table must be filled with appropriate\n\nvalues before the up- scaling functionality can be\n\nused.\n\n"]
    #[inline(always)]
    pub fn scale_lut(&self) -> ScaleLutR {
        ScaleLutR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Entry of lookup table at position scale_lut_addr.\n\nThe lookup table must be filled with appropriate\n\nvalues before the up- scaling functionality can be\n\nused.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn scale_lut(&mut self) -> ScaleLutW<SrszScaleLutSpec> {
        ScaleLutW::new(self, 0)
    }
}
#[doc = "Entry of up-scaling look up table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsz_scale_lut::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsz_scale_lut::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrszScaleLutSpec;
impl crate::RegisterSpec for SrszScaleLutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsz_scale_lut::R`](R) reader structure"]
impl crate::Readable for SrszScaleLutSpec {}
#[doc = "`write(|w| ..)` method takes [`srsz_scale_lut::W`](W) writer structure"]
impl crate::Writable for SrszScaleLutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSZ_SCALE_LUT to value 0"]
impl crate::Resettable for SrszScaleLutSpec {
    const RESET_VALUE: u32 = 0;
}
