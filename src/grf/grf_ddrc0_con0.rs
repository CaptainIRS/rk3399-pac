#[doc = "Register `GRF_DDRC0_CON0` reader"]
pub type R = crate::R<GrfDdrc0Con0Spec>;
#[doc = "Register `GRF_DDRC0_CON0` writer"]
pub type W = crate::W<GrfDdrc0Con0Spec>;
#[doc = "Field `DDR0_ZQ_STATUS_IN` reader - bit control of ddr0_zq_status_in"]
pub type Ddr0ZqStatusInR = crate::FieldReader;
#[doc = "Field `DDR0_ZQ_STATUS_IN` writer - bit control of ddr0_zq_status_in"]
pub type Ddr0ZqStatusInW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR0_LP4_ADDR_DUP` reader - bit conrol of ddr0_lp4_addr_dup"]
pub type Ddr0Lp4AddrDupR = crate::BitReader;
#[doc = "Field `DDR0_LP4_ADDR_DUP` writer - bit conrol of ddr0_lp4_addr_dup"]
pub type Ddr0Lp4AddrDupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_TSEL_EN_POLARITY` reader - bit control of ddr0_tsel_en_polarity"]
pub type Ddr0TselEnPolarityR = crate::BitReader;
#[doc = "Field `DDR0_TSEL_EN_POLARITY` writer - bit control of ddr0_tsel_en_polarity"]
pub type Ddr0TselEnPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_IE_POLARITY` reader - bit control of ddr0_ie_polarity"]
pub type Ddr0IePolarityR = crate::BitReader;
#[doc = "Field `DDR0_IE_POLARITY` writer - bit control of ddr0_ie_polarity"]
pub type Ddr0IePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_IO_CTRL_IE_POLARITY` reader - bit control of ddr0_io_ctrl_ie_polarity"]
pub type Ddr0IoCtrlIePolarityR = crate::BitReader;
#[doc = "Field `DDR0_IO_CTRL_IE_POLARITY` writer - bit control of ddr0_io_ctrl_ie_polarity"]
pub type Ddr0IoCtrlIePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_IO_CTRL_OE_POLARITY` reader - bit control of ddr0_io_ctrl_ie_polarity"]
pub type Ddr0IoCtrlOePolarityR = crate::BitReader;
#[doc = "Field `DDR0_IO_CTRL_OE_POLARITY` writer - bit control of ddr0_io_ctrl_ie_polarity"]
pub type Ddr0IoCtrlOePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_DRAM_CLK_ENABLE_POLARITY` reader - bit control of ddr0_dram_clk_enable_polarity"]
pub type Ddr0DramClkEnablePolarityR = crate::BitReader;
#[doc = "Field `DDR0_DRAM_CLK_ENABLE_POLARITY` writer - bit control of ddr0_dram_clk_enable_polarity"]
pub type Ddr0DramClkEnablePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_OE_POLARITY` reader - bit control of ddr0_oe_polarity"]
pub type Ddr0OePolarityR = crate::BitReader;
#[doc = "Field `DDR0_OE_POLARITY` writer - bit control of ddr0_oe_polarity"]
pub type Ddr0OePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - bit control of ddr0_zq_status_in"]
    #[inline(always)]
    pub fn ddr0_zq_status_in(&self) -> Ddr0ZqStatusInR {
        Ddr0ZqStatusInR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - bit conrol of ddr0_lp4_addr_dup"]
    #[inline(always)]
    pub fn ddr0_lp4_addr_dup(&self) -> Ddr0Lp4AddrDupR {
        Ddr0Lp4AddrDupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - bit control of ddr0_tsel_en_polarity"]
    #[inline(always)]
    pub fn ddr0_tsel_en_polarity(&self) -> Ddr0TselEnPolarityR {
        Ddr0TselEnPolarityR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - bit control of ddr0_ie_polarity"]
    #[inline(always)]
    pub fn ddr0_ie_polarity(&self) -> Ddr0IePolarityR {
        Ddr0IePolarityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - bit control of ddr0_io_ctrl_ie_polarity"]
    #[inline(always)]
    pub fn ddr0_io_ctrl_ie_polarity(&self) -> Ddr0IoCtrlIePolarityR {
        Ddr0IoCtrlIePolarityR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - bit control of ddr0_io_ctrl_ie_polarity"]
    #[inline(always)]
    pub fn ddr0_io_ctrl_oe_polarity(&self) -> Ddr0IoCtrlOePolarityR {
        Ddr0IoCtrlOePolarityR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - bit control of ddr0_dram_clk_enable_polarity"]
    #[inline(always)]
    pub fn ddr0_dram_clk_enable_polarity(&self) -> Ddr0DramClkEnablePolarityR {
        Ddr0DramClkEnablePolarityR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - bit control of ddr0_oe_polarity"]
    #[inline(always)]
    pub fn ddr0_oe_polarity(&self) -> Ddr0OePolarityR {
        Ddr0OePolarityR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit control of ddr0_zq_status_in"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_zq_status_in(&mut self) -> Ddr0ZqStatusInW<GrfDdrc0Con0Spec> {
        Ddr0ZqStatusInW::new(self, 0)
    }
    #[doc = "Bit 2 - bit conrol of ddr0_lp4_addr_dup"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_lp4_addr_dup(&mut self) -> Ddr0Lp4AddrDupW<GrfDdrc0Con0Spec> {
        Ddr0Lp4AddrDupW::new(self, 2)
    }
    #[doc = "Bit 7 - bit control of ddr0_tsel_en_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_tsel_en_polarity(&mut self) -> Ddr0TselEnPolarityW<GrfDdrc0Con0Spec> {
        Ddr0TselEnPolarityW::new(self, 7)
    }
    #[doc = "Bit 8 - bit control of ddr0_ie_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_ie_polarity(&mut self) -> Ddr0IePolarityW<GrfDdrc0Con0Spec> {
        Ddr0IePolarityW::new(self, 8)
    }
    #[doc = "Bit 9 - bit control of ddr0_io_ctrl_ie_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_io_ctrl_ie_polarity(&mut self) -> Ddr0IoCtrlIePolarityW<GrfDdrc0Con0Spec> {
        Ddr0IoCtrlIePolarityW::new(self, 9)
    }
    #[doc = "Bit 10 - bit control of ddr0_io_ctrl_ie_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_io_ctrl_oe_polarity(&mut self) -> Ddr0IoCtrlOePolarityW<GrfDdrc0Con0Spec> {
        Ddr0IoCtrlOePolarityW::new(self, 10)
    }
    #[doc = "Bit 11 - bit control of ddr0_dram_clk_enable_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_dram_clk_enable_polarity(
        &mut self,
    ) -> Ddr0DramClkEnablePolarityW<GrfDdrc0Con0Spec> {
        Ddr0DramClkEnablePolarityW::new(self, 11)
    }
    #[doc = "Bit 12 - bit control of ddr0_oe_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_oe_polarity(&mut self) -> Ddr0OePolarityW<GrfDdrc0Con0Spec> {
        Ddr0OePolarityW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfDdrc0Con0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "ddrc0 control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_ddrc0_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_ddrc0_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfDdrc0Con0Spec;
impl crate::RegisterSpec for GrfDdrc0Con0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_ddrc0_con0::R`](R) reader structure"]
impl crate::Readable for GrfDdrc0Con0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_ddrc0_con0::W`](W) writer structure"]
impl crate::Writable for GrfDdrc0Con0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_DDRC0_CON0 to value 0x1f81"]
impl crate::Resettable for GrfDdrc0Con0Spec {
    const RESET_VALUE: u32 = 0x1f81;
}
