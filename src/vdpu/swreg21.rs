#[doc = "Register `SWREG21` reader"]
pub type R = crate::R<Swreg21Spec>;
#[doc = "Register `SWREG21` writer"]
pub type W = crate::W<Swreg21Spec>;
#[doc = "Field `SW_Y_OUT_ST_ADR` reader - output y component start address\n\nalso the start address of YUYV and RGB"]
pub type SwYOutStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_Y_OUT_ST_ADR` writer - output y component start address\n\nalso the start address of YUYV and RGB"]
pub type SwYOutStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - output y component start address\n\nalso the start address of YUYV and RGB"]
    #[inline(always)]
    pub fn sw_y_out_st_adr(&self) -> SwYOutStAdrR {
        SwYOutStAdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - output y component start address\n\nalso the start address of YUYV and RGB"]
    #[inline(always)]
    #[must_use]
    pub fn sw_y_out_st_adr(&mut self) -> SwYOutStAdrW<Swreg21Spec> {
        SwYOutStAdrW::new(self, 0)
    }
}
#[doc = "Base address for writing post-processed picture luminance/RGB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg21Spec;
impl crate::RegisterSpec for Swreg21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg21::R`](R) reader structure"]
impl crate::Readable for Swreg21Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg21::W`](W) writer structure"]
impl crate::Writable for Swreg21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG21 to value 0"]
impl crate::Resettable for Swreg21Spec {
    const RESET_VALUE: u32 = 0;
}
