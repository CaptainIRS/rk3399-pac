#[doc = "Register `DP_TEST_HBR2_PATTERN` reader"]
pub type R = crate::R<DpTestHbr2PatternSpec>;
#[doc = "Register `DP_TEST_HBR2_PATTERN` writer"]
pub type W = crate::W<DpTestHbr2PatternSpec>;
#[doc = "Field `DP_TEST_HBR2_PATTERN` reader - Hbr2 compliance SR count"]
pub type DpTestHbr2PatternR = crate::FieldReader<u16>;
#[doc = "Field `DP_TEST_HBR2_PATTERN` writer - Hbr2 compliance SR count"]
pub type DpTestHbr2PatternW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Hbr2 compliance SR count"]
    #[inline(always)]
    pub fn dp_test_hbr2_pattern(&self) -> DpTestHbr2PatternR {
        DpTestHbr2PatternR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Hbr2 compliance SR count"]
    #[inline(always)]
    #[must_use]
    pub fn dp_test_hbr2_pattern(&mut self) -> DpTestHbr2PatternW<DpTestHbr2PatternSpec> {
        DpTestHbr2PatternW::new(self, 0)
    }
}
#[doc = "Hbr2 compliance SR count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_test_hbr2_pattern::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_test_hbr2_pattern::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpTestHbr2PatternSpec;
impl crate::RegisterSpec for DpTestHbr2PatternSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_test_hbr2_pattern::R`](R) reader structure"]
impl crate::Readable for DpTestHbr2PatternSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_test_hbr2_pattern::W`](W) writer structure"]
impl crate::Writable for DpTestHbr2PatternSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DP_TEST_HBR2_PATTERN to value 0"]
impl crate::Resettable for DpTestHbr2PatternSpec {
    const RESET_VALUE: u32 = 0;
}
