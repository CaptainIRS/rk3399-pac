#[doc = "Register `SWREG14_HEVC_REFER4_BASE` reader"]
pub type R = crate::R<Swreg14HevcRefer4BaseSpec>;
#[doc = "Register `SWREG14_HEVC_REFER4_BASE` writer"]
pub type W = crate::W<Swreg14HevcRefer4BaseSpec>;
#[doc = "Field `SW_REFER4_BASE` reader - base address for reference picture index 4\n\nbase address for reference picture index 4(the address should be\n\n128bit align)"]
pub type SwRefer4BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER4_BASE` writer - base address for reference picture index 4\n\nbase address for reference picture index 4(the address should be\n\n128bit align)"]
pub type SwRefer4BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for reference picture index 4\n\nbase address for reference picture index 4(the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer4_base(&self) -> SwRefer4BaseR {
        SwRefer4BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for reference picture index 4\n\nbase address for reference picture index 4(the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer4_base(&mut self) -> SwRefer4BaseW<Swreg14HevcRefer4BaseSpec> {
        SwRefer4BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg14_hevc_refer4_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg14_hevc_refer4_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg14HevcRefer4BaseSpec;
impl crate::RegisterSpec for Swreg14HevcRefer4BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg14_hevc_refer4_base::R`](R) reader structure"]
impl crate::Readable for Swreg14HevcRefer4BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg14_hevc_refer4_base::W`](W) writer structure"]
impl crate::Writable for Swreg14HevcRefer4BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG14_HEVC_REFER4_BASE to value 0"]
impl crate::Resettable for Swreg14HevcRefer4BaseSpec {
    const RESET_VALUE: u32 = 0;
}
