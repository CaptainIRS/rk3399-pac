#[doc = "Register `SWREG16_HEVC_REFER6_BASE` reader"]
pub type R = crate::R<Swreg16HevcRefer6BaseSpec>;
#[doc = "Register `SWREG16_HEVC_REFER6_BASE` writer"]
pub type W = crate::W<Swreg16HevcRefer6BaseSpec>;
#[doc = "Field `SW_REFER6_BASE` reader - base address for reference picture index 6\n\nbase address for reference picture index 6(the address should be\n\n128bit align)"]
pub type SwRefer6BaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER6_BASE` writer - base address for reference picture index 6\n\nbase address for reference picture index 6(the address should be\n\n128bit align)"]
pub type SwRefer6BaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for reference picture index 6\n\nbase address for reference picture index 6(the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_refer6_base(&self) -> SwRefer6BaseR {
        SwRefer6BaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for reference picture index 6\n\nbase address for reference picture index 6(the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer6_base(&mut self) -> SwRefer6BaseW<Swreg16HevcRefer6BaseSpec> {
        SwRefer6BaseW::new(self, 4)
    }
}
#[doc = "base address for reference picture index 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg16_hevc_refer6_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg16_hevc_refer6_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg16HevcRefer6BaseSpec;
impl crate::RegisterSpec for Swreg16HevcRefer6BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg16_hevc_refer6_base::R`](R) reader structure"]
impl crate::Readable for Swreg16HevcRefer6BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg16_hevc_refer6_base::W`](W) writer structure"]
impl crate::Writable for Swreg16HevcRefer6BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG16_HEVC_REFER6_BASE to value 0"]
impl crate::Resettable for Swreg16HevcRefer6BaseSpec {
    const RESET_VALUE: u32 = 0;
}
