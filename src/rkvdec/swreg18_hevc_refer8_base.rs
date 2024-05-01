#[doc = "Register `SWREG18_HEVC_REFER8_BASE` reader"]
pub type R = crate::R<Swreg18HevcRefer8BaseSpec>;
#[doc = "Register `SWREG18_HEVC_REFER8_BASE` writer"]
pub type W = crate::W<Swreg18HevcRefer8BaseSpec>;
#[doc = "Field `SW_REFER8_BASE` reader - base address for reference picture index 8\n\nbase address for reference picture index 8(the address should be\n\n128bit align)"]
pub type SwRefer8BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER8_BASE` writer - base address for reference picture index 8\n\nbase address for reference picture index 8(the address should be\n\n128bit align)"]
pub type SwRefer8BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for reference picture index 8\n\nbase address for reference picture index 8(the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer8_base(&self) -> SwRefer8BaseR {
        SwRefer8BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for reference picture index 8\n\nbase address for reference picture index 8(the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer8_base(&mut self) -> SwRefer8BaseW<Swreg18HevcRefer8BaseSpec> {
        SwRefer8BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg18_hevc_refer8_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg18_hevc_refer8_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg18HevcRefer8BaseSpec;
impl crate::RegisterSpec for Swreg18HevcRefer8BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg18_hevc_refer8_base::R`](R) reader structure"]
impl crate::Readable for Swreg18HevcRefer8BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg18_hevc_refer8_base::W`](W) writer structure"]
impl crate::Writable for Swreg18HevcRefer8BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG18_HEVC_REFER8_BASE to value 0"]
impl crate::Resettable for Swreg18HevcRefer8BaseSpec {
    const RESET_VALUE: u32 = 0;
}
