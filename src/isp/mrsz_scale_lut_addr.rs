#[doc = "Register `MRSZ_SCALE_LUT_ADDR` reader"]
pub type R = crate::R<MrszScaleLutAddrSpec>;
#[doc = "Register `MRSZ_SCALE_LUT_ADDR` writer"]
pub type W = crate::W<MrszScaleLutAddrSpec>;
#[doc = "Field `scale_lut_addr` reader - Pointer to entry of lookup table\n\n"]
pub type ScaleLutAddrR = crate::FieldReader;
#[doc = "Field `scale_lut_addr` writer - Pointer to entry of lookup table\n\n"]
pub type ScaleLutAddrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Pointer to entry of lookup table\n\n"]
    #[inline(always)]
    pub fn scale_lut_addr(&self) -> ScaleLutAddrR {
        ScaleLutAddrR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pointer to entry of lookup table\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn scale_lut_addr(&mut self) -> ScaleLutAddrW<MrszScaleLutAddrSpec> {
        ScaleLutAddrW::new(self, 0)
    }
}
#[doc = "Address pointer of up-scaling look up table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_scale_lut_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mrsz_scale_lut_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszScaleLutAddrSpec;
impl crate::RegisterSpec for MrszScaleLutAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_scale_lut_addr::R`](R) reader structure"]
impl crate::Readable for MrszScaleLutAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mrsz_scale_lut_addr::W`](W) writer structure"]
impl crate::Writable for MrszScaleLutAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRSZ_SCALE_LUT_ADDR to value 0"]
impl crate::Resettable for MrszScaleLutAddrSpec {
    const RESET_VALUE: u32 = 0;
}
