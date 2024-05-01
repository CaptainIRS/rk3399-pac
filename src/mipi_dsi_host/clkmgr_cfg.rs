#[doc = "Register `CLKMGR_CFG` reader"]
pub type R = crate::R<ClkmgrCfgSpec>;
#[doc = "Register `CLKMGR_CFG` writer"]
pub type W = crate::W<ClkmgrCfgSpec>;
#[doc = "Field `TX_ESC_CLK_DIVISION` reader - tx_esc_clk_division\n\nThis field indicates the division factor for the TX Escape clock source\n\n(lanebyteclk). The values 0 and 1 stop the TX_ESC clock\n\ngeneration."]
pub type TxEscClkDivisionR = crate::FieldReader;
#[doc = "Field `TX_ESC_CLK_DIVISION` writer - tx_esc_clk_division\n\nThis field indicates the division factor for the TX Escape clock source\n\n(lanebyteclk). The values 0 and 1 stop the TX_ESC clock\n\ngeneration."]
pub type TxEscClkDivisionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TO_CLK_DIVISION` reader - to_clk_division\n\nThis field indicates the division factor for the Time Out clock used as\n\nthe timing unit in the configuration of HS to LP and LP to HS\n\ntransition\n\nerror."]
pub type ToClkDivisionR = crate::FieldReader;
#[doc = "Field `TO_CLK_DIVISION` writer - to_clk_division\n\nThis field indicates the division factor for the Time Out clock used as\n\nthe timing unit in the configuration of HS to LP and LP to HS\n\ntransition\n\nerror."]
pub type ToClkDivisionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - tx_esc_clk_division\n\nThis field indicates the division factor for the TX Escape clock source\n\n(lanebyteclk). The values 0 and 1 stop the TX_ESC clock\n\ngeneration."]
    #[inline(always)]
    pub fn tx_esc_clk_division(&self) -> TxEscClkDivisionR {
        TxEscClkDivisionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - to_clk_division\n\nThis field indicates the division factor for the Time Out clock used as\n\nthe timing unit in the configuration of HS to LP and LP to HS\n\ntransition\n\nerror."]
    #[inline(always)]
    pub fn to_clk_division(&self) -> ToClkDivisionR {
        ToClkDivisionR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tx_esc_clk_division\n\nThis field indicates the division factor for the TX Escape clock source\n\n(lanebyteclk). The values 0 and 1 stop the TX_ESC clock\n\ngeneration."]
    #[inline(always)]
    #[must_use]
    pub fn tx_esc_clk_division(&mut self) -> TxEscClkDivisionW<ClkmgrCfgSpec> {
        TxEscClkDivisionW::new(self, 0)
    }
    #[doc = "Bits 8:15 - to_clk_division\n\nThis field indicates the division factor for the Time Out clock used as\n\nthe timing unit in the configuration of HS to LP and LP to HS\n\ntransition\n\nerror."]
    #[inline(always)]
    #[must_use]
    pub fn to_clk_division(&mut self) -> ToClkDivisionW<ClkmgrCfgSpec> {
        ToClkDivisionW::new(self, 8)
    }
}
#[doc = "Internal Clock Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkmgr_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkmgr_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkmgrCfgSpec;
impl crate::RegisterSpec for ClkmgrCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkmgr_cfg::R`](R) reader structure"]
impl crate::Readable for ClkmgrCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkmgr_cfg::W`](W) writer structure"]
impl crate::Writable for ClkmgrCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKMGR_CFG to value 0"]
impl crate::Resettable for ClkmgrCfgSpec {
    const RESET_VALUE: u32 = 0;
}
