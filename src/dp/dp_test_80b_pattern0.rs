#[doc = "Register `DP_TEST_80B_PATTERN0` reader"]
pub type R = crate::R<DpTest80bPattern0Spec>;
#[doc = "Register `DP_TEST_80B_PATTERN0` writer"]
pub type W = crate::W<DpTest80bPattern0Spec>;
#[doc = "Field `DP_TEST_80B_PATTERN0` reader - DP test 80bit pattern0\\[29:0\\]"]
pub type DpTest80bPattern0R = crate::FieldReader<u32>;
#[doc = "Field `DP_TEST_80B_PATTERN0` writer - DP test 80bit pattern0\\[29:0\\]"]
pub type DpTest80bPattern0W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - DP test 80bit pattern0\\[29:0\\]"]
    #[inline(always)]
    pub fn dp_test_80b_pattern0(&self) -> DpTest80bPattern0R {
        DpTest80bPattern0R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - DP test 80bit pattern0\\[29:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dp_test_80b_pattern0(&mut self) -> DpTest80bPattern0W<DpTest80bPattern0Spec> {
        DpTest80bPattern0W::new(self, 0)
    }
}
#[doc = "80b pattern \\[29:0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_test_80b_pattern0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_test_80b_pattern0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpTest80bPattern0Spec;
impl crate::RegisterSpec for DpTest80bPattern0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_test_80b_pattern0::R`](R) reader structure"]
impl crate::Readable for DpTest80bPattern0Spec {}
#[doc = "`write(|w| ..)` method takes [`dp_test_80b_pattern0::W`](W) writer structure"]
impl crate::Writable for DpTest80bPattern0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DP_TEST_80B_PATTERN0 to value 0"]
impl crate::Resettable for DpTest80bPattern0Spec {
    const RESET_VALUE: u32 = 0;
}
