#[doc = "Register `LP_WR_TO_CNT` reader"]
pub type R = crate::R<LpWrToCntSpec>;
#[doc = "Register `LP_WR_TO_CNT` writer"]
pub type W = crate::W<LpWrToCntSpec>;
#[doc = "Field `LP_WR_TO_CNT` reader - lp_wr_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the\n\nlink still, after sending a low-power write operation. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
pub type LpWrToCntR = crate::FieldReader<u16>;
#[doc = "Field `LP_WR_TO_CNT` writer - lp_wr_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the\n\nlink still, after sending a low-power write operation. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
pub type LpWrToCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - lp_wr_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the\n\nlink still, after sending a low-power write operation. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    pub fn lp_wr_to_cnt(&self) -> LpWrToCntR {
        LpWrToCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - lp_wr_to_cnt\n\nThis field sets a period for which the DSI HOST keeps the\n\nlink still, after sending a low-power write operation. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn lp_wr_to_cnt(&mut self) -> LpWrToCntW<LpWrToCntSpec> {
        LpWrToCntW::new(self, 0)
    }
}
#[doc = "Peripheral Response Timeout Definition after Lo\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_wr_to_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_wr_to_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpWrToCntSpec;
impl crate::RegisterSpec for LpWrToCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_wr_to_cnt::R`](R) reader structure"]
impl crate::Readable for LpWrToCntSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_wr_to_cnt::W`](W) writer structure"]
impl crate::Writable for LpWrToCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_WR_TO_CNT to value 0"]
impl crate::Resettable for LpWrToCntSpec {
    const RESET_VALUE: u32 = 0;
}
