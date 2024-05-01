#[doc = "Register `SWREG19_HEVC_REFER9_BASE` reader"]
pub type R = crate::R<Swreg19HevcRefer9BaseSpec>;
#[doc = "Register `SWREG19_HEVC_REFER9_BASE` writer"]
pub type W = crate::W<Swreg19HevcRefer9BaseSpec>;
#[doc = "Field `SW_REFER9_BASE` reader - base address for reference picture index 9\n\nbase address for reference picture index 9(the address should be\n\n128bit align)"]
pub type SwRefer9BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER9_BASE` writer - base address for reference picture index 9\n\nbase address for reference picture index 9(the address should be\n\n128bit align)"]
pub type SwRefer9BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for reference picture index 9\n\nbase address for reference picture index 9(the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer9_base(&self) -> SwRefer9BaseR {
        SwRefer9BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for reference picture index 9\n\nbase address for reference picture index 9(the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer9_base(&mut self) -> SwRefer9BaseW<Swreg19HevcRefer9BaseSpec> {
        SwRefer9BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg19_hevc_refer9_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg19_hevc_refer9_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg19HevcRefer9BaseSpec;
impl crate::RegisterSpec for Swreg19HevcRefer9BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg19_hevc_refer9_base::R`](R) reader structure"]
impl crate::Readable for Swreg19HevcRefer9BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg19_hevc_refer9_base::W`](W) writer structure"]
impl crate::Writable for Swreg19HevcRefer9BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG19_HEVC_REFER9_BASE to value 0"]
impl crate::Resettable for Swreg19HevcRefer9BaseSpec {
    const RESET_VALUE: u32 = 0;
}
