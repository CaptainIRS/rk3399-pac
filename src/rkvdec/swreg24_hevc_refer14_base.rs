#[doc = "Register `SWREG24_HEVC_REFER14_BASE` reader"]
pub type R = crate::R<Swreg24HevcRefer14BaseSpec>;
#[doc = "Register `SWREG24_HEVC_REFER14_BASE` writer"]
pub type W = crate::W<Swreg24HevcRefer14BaseSpec>;
#[doc = "Field `SW_REFER14_BASE` reader - base address for reference picture index 14\n\nbase address for reference picture index 14(the address should be\n\n128bit align)"]
pub type SwRefer14BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER14_BASE` writer - base address for reference picture index 14\n\nbase address for reference picture index 14(the address should be\n\n128bit align)"]
pub type SwRefer14BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for reference picture index 14\n\nbase address for reference picture index 14(the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer14_base(&self) -> SwRefer14BaseR {
        SwRefer14BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for reference picture index 14\n\nbase address for reference picture index 14(the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer14_base(&mut self) -> SwRefer14BaseW<Swreg24HevcRefer14BaseSpec> {
        SwRefer14BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg24_hevc_refer14_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg24_hevc_refer14_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg24HevcRefer14BaseSpec;
impl crate::RegisterSpec for Swreg24HevcRefer14BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg24_hevc_refer14_base::R`](R) reader structure"]
impl crate::Readable for Swreg24HevcRefer14BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg24_hevc_refer14_base::W`](W) writer structure"]
impl crate::Writable for Swreg24HevcRefer14BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG24_HEVC_REFER14_BASE to value 0"]
impl crate::Resettable for Swreg24HevcRefer14BaseSpec {
    const RESET_VALUE: u32 = 0;
}
