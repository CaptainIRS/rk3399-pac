#[doc = "Register `SWREG15_HEVC_REFER5_BASE` reader"]
pub type R = crate::R<Swreg15HevcRefer5BaseSpec>;
#[doc = "Register `SWREG15_HEVC_REFER5_BASE` writer"]
pub type W = crate::W<Swreg15HevcRefer5BaseSpec>;
#[doc = "Field `SW_REFER5_BASE` reader - base address for reference picture index 5\n\nbase address for reference picture index 5(the address should be\n\n128bit align)"]
pub type SwRefer5BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER5_BASE` writer - base address for reference picture index 5\n\nbase address for reference picture index 5(the address should be\n\n128bit align)"]
pub type SwRefer5BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for reference picture index 5\n\nbase address for reference picture index 5(the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer5_base(&self) -> SwRefer5BaseR {
        SwRefer5BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for reference picture index 5\n\nbase address for reference picture index 5(the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer5_base(&mut self) -> SwRefer5BaseW<Swreg15HevcRefer5BaseSpec> {
        SwRefer5BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg15_hevc_refer5_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg15_hevc_refer5_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg15HevcRefer5BaseSpec;
impl crate::RegisterSpec for Swreg15HevcRefer5BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg15_hevc_refer5_base::R`](R) reader structure"]
impl crate::Readable for Swreg15HevcRefer5BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg15_hevc_refer5_base::W`](W) writer structure"]
impl crate::Writable for Swreg15HevcRefer5BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG15_HEVC_REFER5_BASE to value 0"]
impl crate::Resettable for Swreg15HevcRefer5BaseSpec {
    const RESET_VALUE: u32 = 0;
}
