#[doc = "Register `CRU_CLKSEL_CON43` reader"]
pub type R = crate::R<CruClkselCon43Spec>;
#[doc = "Register `CRU_CLKSEL_CON43` writer"]
pub type W = crate::W<CruClkselCon43Spec>;
#[doc = "Field `PCLK_VIO_DIV_CON` reader - pclk_vio divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkVioDivConR = crate::FieldReader;
#[doc = "Field `PCLK_VIO_DIV_CON` writer - pclk_vio divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkVioDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HCLK_HDCP_DIV_CON` reader - hclk_hdcp divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkHdcpDivConR = crate::FieldReader;
#[doc = "Field `HCLK_HDCP_DIV_CON` writer - hclk_hdcp divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkHdcpDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PCLK_HDCP_DIV_CON` reader - pclk_hdcp divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkHdcpDivConR = crate::FieldReader;
#[doc = "Field `PCLK_HDCP_DIV_CON` writer - pclk_hdcp divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkHdcpDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - pclk_vio divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_vio_div_con(&self) -> PclkVioDivConR {
        PclkVioDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - hclk_hdcp divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_hdcp_div_con(&self) -> HclkHdcpDivConR {
        HclkHdcpDivConR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - pclk_hdcp divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_hdcp_div_con(&self) -> PclkHdcpDivConR {
        PclkHdcpDivConR::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - pclk_vio divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_vio_div_con(&mut self) -> PclkVioDivConW<CruClkselCon43Spec> {
        PclkVioDivConW::new(self, 0)
    }
    #[doc = "Bits 5:9 - hclk_hdcp divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_hdcp_div_con(&mut self) -> HclkHdcpDivConW<CruClkselCon43Spec> {
        HclkHdcpDivConW::new(self, 5)
    }
    #[doc = "Bits 10:14 - pclk_hdcp divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_hdcp_div_con(&mut self) -> PclkHdcpDivConW<CruClkselCon43Spec> {
        PclkHdcpDivConW::new(self, 10)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon43Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon43Spec;
impl crate::RegisterSpec for CruClkselCon43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con43::R`](R) reader structure"]
impl crate::Readable for CruClkselCon43Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con43::W`](W) writer structure"]
impl crate::Writable for CruClkselCon43Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON43 to value 0x0421"]
impl crate::Resettable for CruClkselCon43Spec {
    const RESET_VALUE: u32 = 0x0421;
}
