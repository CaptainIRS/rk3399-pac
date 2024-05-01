#[doc = "Register `HS_RD_TO_CNT` reader"]
pub type R = crate::R<HsRdToCntSpec>;
#[doc = "Register `HS_RD_TO_CNT` writer"]
pub type W = crate::W<HsRdToCntSpec>;
#[doc = "Field `HS_RD_TO_CNT` reader - hs_rd_to_cnt\n\nThis field sets a period for which the DWC_mipi_dsi_host keeps the\n\nlink still, after sending a high-speed read operation. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
pub type HsRdToCntR = crate::FieldReader<u16>;
#[doc = "Field `HS_RD_TO_CNT` writer - hs_rd_to_cnt\n\nThis field sets a period for which the DWC_mipi_dsi_host keeps the\n\nlink still, after sending a high-speed read operation. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
pub type HsRdToCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - hs_rd_to_cnt\n\nThis field sets a period for which the DWC_mipi_dsi_host keeps the\n\nlink still, after sending a high-speed read operation. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    pub fn hs_rd_to_cnt(&self) -> HsRdToCntR {
        HsRdToCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - hs_rd_to_cnt\n\nThis field sets a period for which the DWC_mipi_dsi_host keeps the\n\nlink still, after sending a high-speed read operation. This period is\n\nmeasured in cycles of lanebyteclk. The counting starts when the\n\nD-PHY enters the Stop state and causes no interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn hs_rd_to_cnt(&mut self) -> HsRdToCntW<HsRdToCntSpec> {
        HsRdToCntW::new(self, 0)
    }
}
#[doc = "Peripheral Response Timeout Definition after Hi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_rd_to_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_rd_to_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsRdToCntSpec;
impl crate::RegisterSpec for HsRdToCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_rd_to_cnt::R`](R) reader structure"]
impl crate::Readable for HsRdToCntSpec {}
#[doc = "`write(|w| ..)` method takes [`hs_rd_to_cnt::W`](W) writer structure"]
impl crate::Writable for HsRdToCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HS_RD_TO_CNT to value 0"]
impl crate::Resettable for HsRdToCntSpec {
    const RESET_VALUE: u32 = 0;
}
