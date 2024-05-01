#[doc = "Register `SWREG21_HEVC_REFER11_BASE` reader"]
pub type R = crate::R<Swreg21HevcRefer11BaseSpec>;
#[doc = "Register `SWREG21_HEVC_REFER11_BASE` writer"]
pub type W = crate::W<Swreg21HevcRefer11BaseSpec>;
#[doc = "Field `SW_REFER11_BASE` reader - base address for reference picture index 11\n\nbase address for reference picture index 11(the address should be\n\n128bit align)"]
pub type SwRefer11BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER11_BASE` writer - base address for reference picture index 11\n\nbase address for reference picture index 11(the address should be\n\n128bit align)"]
pub type SwRefer11BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for reference picture index 11\n\nbase address for reference picture index 11(the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer11_base(&self) -> SwRefer11BaseR {
        SwRefer11BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for reference picture index 11\n\nbase address for reference picture index 11(the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer11_base(&mut self) -> SwRefer11BaseW<Swreg21HevcRefer11BaseSpec> {
        SwRefer11BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg21_hevc_refer11_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg21_hevc_refer11_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg21HevcRefer11BaseSpec;
impl crate::RegisterSpec for Swreg21HevcRefer11BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg21_hevc_refer11_base::R`](R) reader structure"]
impl crate::Readable for Swreg21HevcRefer11BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg21_hevc_refer11_base::W`](W) writer structure"]
impl crate::Writable for Swreg21HevcRefer11BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG21_HEVC_REFER11_BASE to value 0"]
impl crate::Resettable for Swreg21HevcRefer11BaseSpec {
    const RESET_VALUE: u32 = 0;
}
