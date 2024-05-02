#[doc = "Register `MRSZ_SCALE_LUT` reader"]
pub type R = crate::R<MrszScaleLutSpec>;
#[doc = "Register `MRSZ_SCALE_LUT` writer"]
pub type W = crate::W<MrszScaleLutSpec>;
#[doc = "Field `scale_lut` reader - Entry of lookup table at position scale_lut_addr. The\n\nlookup table must be filled with appropriate values before\n\nthe up- scaling functionality can be used.\n\n"]
pub type ScaleLutR = crate::FieldReader;
#[doc = "Field `scale_lut` writer - Entry of lookup table at position scale_lut_addr. The\n\nlookup table must be filled with appropriate values before\n\nthe up- scaling functionality can be used.\n\n"]
pub type ScaleLutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Entry of lookup table at position scale_lut_addr. The\n\nlookup table must be filled with appropriate values before\n\nthe up- scaling functionality can be used.\n\n"]
    #[inline(always)]
    pub fn scale_lut(&self) -> ScaleLutR {
        ScaleLutR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Entry of lookup table at position scale_lut_addr. The\n\nlookup table must be filled with appropriate values before\n\nthe up- scaling functionality can be used.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn scale_lut(&mut self) -> ScaleLutW<MrszScaleLutSpec> {
        ScaleLutW::new(self, 0)
    }
}
#[doc = "Entry of up-scaling look up table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_lut::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_lut::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszScaleLutSpec;
impl crate::RegisterSpec for MrszScaleLutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_scale_lut::R`](R) reader structure"]
impl crate::Readable for MrszScaleLutSpec {}
#[doc = "`write(|w| ..)` method takes [`mrsz_scale_lut::W`](W) writer structure"]
impl crate::Writable for MrszScaleLutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRSZ_SCALE_LUT to value 0"]
impl crate::Resettable for MrszScaleLutSpec {
    const RESET_VALUE: u32 = 0;
}
