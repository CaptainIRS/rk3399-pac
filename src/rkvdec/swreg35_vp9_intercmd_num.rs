#[doc = "Register `SWREG35_VP9_INTERCMD_NUM` reader"]
pub type R = crate::R<Swreg35Vp9IntercmdNumSpec>;
#[doc = "Register `SWREG35_VP9_INTERCMD_NUM` writer"]
pub type W = crate::W<Swreg35Vp9IntercmdNumSpec>;
#[doc = "Field `SW_VP9_INTERCMD_NUM` reader - sw_vp9_intercmd_num\n\nwhen rlc_mode is 1'b1, for sw_vp9_intercmd_num\n\nit's unit is 128bit\n\nit count from 1"]
pub type SwVp9IntercmdNumR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9_INTERCMD_NUM` writer - sw_vp9_intercmd_num\n\nwhen rlc_mode is 1'b1, for sw_vp9_intercmd_num\n\nit's unit is 128bit\n\nit count from 1"]
pub type SwVp9IntercmdNumW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - sw_vp9_intercmd_num\n\nwhen rlc_mode is 1'b1, for sw_vp9_intercmd_num\n\nit's unit is 128bit\n\nit count from 1"]
    #[inline(always)]
    pub fn sw_vp9_intercmd_num(&self) -> SwVp9IntercmdNumR {
        SwVp9IntercmdNumR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - sw_vp9_intercmd_num\n\nwhen rlc_mode is 1'b1, for sw_vp9_intercmd_num\n\nit's unit is 128bit\n\nit count from 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_intercmd_num(&mut self) -> SwVp9IntercmdNumW<Swreg35Vp9IntercmdNumSpec> {
        SwVp9IntercmdNumW::new(self, 0)
    }
}
#[doc = "vp9 intercmd num\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg35_vp9_intercmd_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg35_vp9_intercmd_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg35Vp9IntercmdNumSpec;
impl crate::RegisterSpec for Swreg35Vp9IntercmdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg35_vp9_intercmd_num::R`](R) reader structure"]
impl crate::Readable for Swreg35Vp9IntercmdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg35_vp9_intercmd_num::W`](W) writer structure"]
impl crate::Writable for Swreg35Vp9IntercmdNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG35_VP9_INTERCMD_NUM to value 0"]
impl crate::Resettable for Swreg35Vp9IntercmdNumSpec {
    const RESET_VALUE: u32 = 0;
}
