#[doc = "Register `SWREG23_HEVC_REFER13_BASE` reader"]
pub type R = crate::R<Swreg23HevcRefer13BaseSpec>;
#[doc = "Register `SWREG23_HEVC_REFER13_BASE` writer"]
pub type W = crate::W<Swreg23HevcRefer13BaseSpec>;
#[doc = "Field `SW_REFER13_BASE` reader - base address for reference picture index 13\n\nbase address for reference picture index 13(the address should be\n\n128bit align)"]
pub type SwRefer13BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER13_BASE` writer - base address for reference picture index 13\n\nbase address for reference picture index 13(the address should be\n\n128bit align)"]
pub type SwRefer13BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for reference picture index 13\n\nbase address for reference picture index 13(the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer13_base(&self) -> SwRefer13BaseR {
        SwRefer13BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for reference picture index 13\n\nbase address for reference picture index 13(the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer13_base(&mut self) -> SwRefer13BaseW<Swreg23HevcRefer13BaseSpec> {
        SwRefer13BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg23_hevc_refer13_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg23_hevc_refer13_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg23HevcRefer13BaseSpec;
impl crate::RegisterSpec for Swreg23HevcRefer13BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg23_hevc_refer13_base::R`](R) reader structure"]
impl crate::Readable for Swreg23HevcRefer13BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg23_hevc_refer13_base::W`](W) writer structure"]
impl crate::Writable for Swreg23HevcRefer13BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG23_HEVC_REFER13_BASE to value 0"]
impl crate::Resettable for Swreg23HevcRefer13BaseSpec {
    const RESET_VALUE: u32 = 0;
}
