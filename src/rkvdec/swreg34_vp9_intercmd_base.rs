#[doc = "Register `SWREG34_VP9_INTERCMD_BASE` reader"]
pub type R = crate::R<Swreg34Vp9IntercmdBaseSpec>;
#[doc = "Register `SWREG34_VP9_INTERCMD_BASE` writer"]
pub type W = crate::W<Swreg34Vp9IntercmdBaseSpec>;
#[doc = "Field `SW_VP9_INTERCMD_BASE` reader - vp9 intercmd base addr\n\nvp9 inter command base addr, when sw_rlc_mode is 1'b1;\n\nwhen sw_dec_mode is VP9 and sw_rlc_mode is 1'b1, when read\n\nthis register, after a frame is decoded ready or error (stream error ,\n\ntime out , bus error) , it is the end address of the intercmd"]
pub type SwVp9IntercmdBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9_INTERCMD_BASE` writer - vp9 intercmd base addr\n\nvp9 inter command base addr, when sw_rlc_mode is 1'b1;\n\nwhen sw_dec_mode is VP9 and sw_rlc_mode is 1'b1, when read\n\nthis register, after a frame is decoded ready or error (stream error ,\n\ntime out , bus error) , it is the end address of the intercmd"]
pub type SwVp9IntercmdBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - vp9 intercmd base addr\n\nvp9 inter command base addr, when sw_rlc_mode is 1'b1;\n\nwhen sw_dec_mode is VP9 and sw_rlc_mode is 1'b1, when read\n\nthis register, after a frame is decoded ready or error (stream error ,\n\ntime out , bus error) , it is the end address of the intercmd"]
    #[inline(always)]
    pub fn sw_vp9_intercmd_base(&self) -> SwVp9IntercmdBaseR {
        SwVp9IntercmdBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - vp9 intercmd base addr\n\nvp9 inter command base addr, when sw_rlc_mode is 1'b1;\n\nwhen sw_dec_mode is VP9 and sw_rlc_mode is 1'b1, when read\n\nthis register, after a frame is decoded ready or error (stream error ,\n\ntime out , bus error) , it is the end address of the intercmd"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_intercmd_base(&mut self) -> SwVp9IntercmdBaseW<Swreg34Vp9IntercmdBaseSpec> {
        SwVp9IntercmdBaseW::new(self, 4)
    }
}
#[doc = "inter cmd base addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg34_vp9_intercmd_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg34_vp9_intercmd_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg34Vp9IntercmdBaseSpec;
impl crate::RegisterSpec for Swreg34Vp9IntercmdBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg34_vp9_intercmd_base::R`](R) reader structure"]
impl crate::Readable for Swreg34Vp9IntercmdBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg34_vp9_intercmd_base::W`](W) writer structure"]
impl crate::Writable for Swreg34Vp9IntercmdBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG34_VP9_INTERCMD_BASE to value 0"]
impl crate::Resettable for Swreg34Vp9IntercmdBaseSpec {
    const RESET_VALUE: u32 = 0;
}
