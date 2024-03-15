#[doc = "Register `DP_TEST_80B_PATTERN2` reader"]
pub type R = crate::R<DpTest80bPattern2Spec>;
#[doc = "Register `DP_TEST_80B_PATTERN2` writer"]
pub type W = crate::W<DpTest80bPattern2Spec>;
#[doc = "Field `DP_TEST_80B_PATTERN2` reader - DP test 80bit pattern0\\[79:60\\]"]
pub type DpTest80bPattern2R = crate::FieldReader<u32>;
#[doc = "Field `DP_TEST_80B_PATTERN2` writer - DP test 80bit pattern0\\[79:60\\]"]
pub type DpTest80bPattern2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - DP test 80bit pattern0\\[79:60\\]"]
    #[inline(always)]
    pub fn dp_test_80b_pattern2(&self) -> DpTest80bPattern2R {
        DpTest80bPattern2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - DP test 80bit pattern0\\[79:60\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dp_test_80b_pattern2(&mut self) -> DpTest80bPattern2W<DpTest80bPattern2Spec> {
        DpTest80bPattern2W::new(self, 0)
    }
}
#[doc = "80b pattern \\[79:60\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_test_80b_pattern2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_test_80b_pattern2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpTest80bPattern2Spec;
impl crate::RegisterSpec for DpTest80bPattern2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_test_80b_pattern2::R`](R) reader structure"]
impl crate::Readable for DpTest80bPattern2Spec {}
#[doc = "`write(|w| ..)` method takes [`dp_test_80b_pattern2::W`](W) writer structure"]
impl crate::Writable for DpTest80bPattern2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DP_TEST_80B_PATTERN2 to value 0"]
impl crate::Resettable for DpTest80bPattern2Spec {
    const RESET_VALUE: u32 = 0;
}
