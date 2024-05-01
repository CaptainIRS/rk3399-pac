#[doc = "Register `SWREG40_CUR_POC` reader"]
pub type R = crate::R<Swreg40CurPocSpec>;
#[doc = "Register `SWREG40_CUR_POC` writer"]
pub type W = crate::W<Swreg40CurPocSpec>;
#[doc = "Field `SW_CUR_POC` reader - the poc of the cur picture\n\nthe poc of the cur picture\n\nfor H264, it may be cur frame poc or cur top field poc"]
pub type SwCurPocR = crate::FieldReader<u32>;
#[doc = "Field `SW_CUR_POC` writer - the poc of the cur picture\n\nthe poc of the cur picture\n\nfor H264, it may be cur frame poc or cur top field poc"]
pub type SwCurPocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of the cur picture\n\nthe poc of the cur picture\n\nfor H264, it may be cur frame poc or cur top field poc"]
    #[inline(always)]
    pub fn sw_cur_poc(&self) -> SwCurPocR {
        SwCurPocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of the cur picture\n\nthe poc of the cur picture\n\nfor H264, it may be cur frame poc or cur top field poc"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cur_poc(&mut self) -> SwCurPocW<Swreg40CurPocSpec> {
        SwCurPocW::new(self, 0)
    }
}
#[doc = "the poc of cur picture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg40_cur_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg40_cur_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg40CurPocSpec;
impl crate::RegisterSpec for Swreg40CurPocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg40_cur_poc::R`](R) reader structure"]
impl crate::Readable for Swreg40CurPocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg40_cur_poc::W`](W) writer structure"]
impl crate::Writable for Swreg40CurPocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG40_CUR_POC to value 0"]
impl crate::Resettable for Swreg40CurPocSpec {
    const RESET_VALUE: u32 = 0;
}
