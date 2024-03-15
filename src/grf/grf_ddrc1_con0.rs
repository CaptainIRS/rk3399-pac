#[doc = "Register `GRF_DDRC1_CON0` reader"]
pub type R = crate::R<GrfDdrc1Con0Spec>;
#[doc = "Register `GRF_DDRC1_CON0` writer"]
pub type W = crate::W<GrfDdrc1Con0Spec>;
#[doc = "Field `DDR1_ZQ_STATUS_IN` reader - bit control of ddr1_zq_status_in"]
pub type Ddr1ZqStatusInR = crate::FieldReader;
#[doc = "Field `DDR1_ZQ_STATUS_IN` writer - bit control of ddr1_zq_status_in"]
pub type Ddr1ZqStatusInW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR1_LP4_ADDR_DUP` reader - bit control of ddr1_lp4_addr_dup"]
pub type Ddr1Lp4AddrDupR = crate::BitReader;
#[doc = "Field `DDR1_LP4_ADDR_DUP` writer - bit control of ddr1_lp4_addr_dup"]
pub type Ddr1Lp4AddrDupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_TSEL_EN_POLARITY` reader - bit control of ddr1_tsel_en_polarity"]
pub type Ddr1TselEnPolarityR = crate::BitReader;
#[doc = "Field `DDR1_TSEL_EN_POLARITY` writer - bit control of ddr1_tsel_en_polarity"]
pub type Ddr1TselEnPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_IE_POLARITY` reader - bit control of ddr1_ie_polarity"]
pub type Ddr1IePolarityR = crate::BitReader;
#[doc = "Field `DDR1_IE_POLARITY` writer - bit control of ddr1_ie_polarity"]
pub type Ddr1IePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_IO_CTRL_IE_POLARITY` reader - bit control of ddr1_io_ctrl_ie_polarity"]
pub type Ddr1IoCtrlIePolarityR = crate::BitReader;
#[doc = "Field `DDR1_IO_CTRL_IE_POLARITY` writer - bit control of ddr1_io_ctrl_ie_polarity"]
pub type Ddr1IoCtrlIePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_IO_CTRL_OE_POLARITY` reader - bit control of ddr1_io_ctrl_oe_polarity"]
pub type Ddr1IoCtrlOePolarityR = crate::BitReader;
#[doc = "Field `DDR1_IO_CTRL_OE_POLARITY` writer - bit control of ddr1_io_ctrl_oe_polarity"]
pub type Ddr1IoCtrlOePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_DRAM_CLK_ENABLE_POLARITY` reader - bit control of ddr1_dram_clk_enable_polarity"]
pub type Ddr1DramClkEnablePolarityR = crate::BitReader;
#[doc = "Field `DDR1_DRAM_CLK_ENABLE_POLARITY` writer - bit control of ddr1_dram_clk_enable_polarity"]
pub type Ddr1DramClkEnablePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_OE_POLARITY` reader - bit control of ddr1_oe_polarity"]
pub type Ddr1OePolarityR = crate::BitReader;
#[doc = "Field `DDR1_OE_POLARITY` writer - bit control of ddr1_oe_polarity"]
pub type Ddr1OePolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - bit control of ddr1_zq_status_in"]
    #[inline(always)]
    pub fn ddr1_zq_status_in(&self) -> Ddr1ZqStatusInR {
        Ddr1ZqStatusInR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - bit control of ddr1_lp4_addr_dup"]
    #[inline(always)]
    pub fn ddr1_lp4_addr_dup(&self) -> Ddr1Lp4AddrDupR {
        Ddr1Lp4AddrDupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - bit control of ddr1_tsel_en_polarity"]
    #[inline(always)]
    pub fn ddr1_tsel_en_polarity(&self) -> Ddr1TselEnPolarityR {
        Ddr1TselEnPolarityR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - bit control of ddr1_ie_polarity"]
    #[inline(always)]
    pub fn ddr1_ie_polarity(&self) -> Ddr1IePolarityR {
        Ddr1IePolarityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - bit control of ddr1_io_ctrl_ie_polarity"]
    #[inline(always)]
    pub fn ddr1_io_ctrl_ie_polarity(&self) -> Ddr1IoCtrlIePolarityR {
        Ddr1IoCtrlIePolarityR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - bit control of ddr1_io_ctrl_oe_polarity"]
    #[inline(always)]
    pub fn ddr1_io_ctrl_oe_polarity(&self) -> Ddr1IoCtrlOePolarityR {
        Ddr1IoCtrlOePolarityR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - bit control of ddr1_dram_clk_enable_polarity"]
    #[inline(always)]
    pub fn ddr1_dram_clk_enable_polarity(&self) -> Ddr1DramClkEnablePolarityR {
        Ddr1DramClkEnablePolarityR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - bit control of ddr1_oe_polarity"]
    #[inline(always)]
    pub fn ddr1_oe_polarity(&self) -> Ddr1OePolarityR {
        Ddr1OePolarityR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit control of ddr1_zq_status_in"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_zq_status_in(&mut self) -> Ddr1ZqStatusInW<GrfDdrc1Con0Spec> {
        Ddr1ZqStatusInW::new(self, 0)
    }
    #[doc = "Bit 2 - bit control of ddr1_lp4_addr_dup"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_lp4_addr_dup(&mut self) -> Ddr1Lp4AddrDupW<GrfDdrc1Con0Spec> {
        Ddr1Lp4AddrDupW::new(self, 2)
    }
    #[doc = "Bit 7 - bit control of ddr1_tsel_en_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_tsel_en_polarity(&mut self) -> Ddr1TselEnPolarityW<GrfDdrc1Con0Spec> {
        Ddr1TselEnPolarityW::new(self, 7)
    }
    #[doc = "Bit 8 - bit control of ddr1_ie_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_ie_polarity(&mut self) -> Ddr1IePolarityW<GrfDdrc1Con0Spec> {
        Ddr1IePolarityW::new(self, 8)
    }
    #[doc = "Bit 9 - bit control of ddr1_io_ctrl_ie_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_io_ctrl_ie_polarity(&mut self) -> Ddr1IoCtrlIePolarityW<GrfDdrc1Con0Spec> {
        Ddr1IoCtrlIePolarityW::new(self, 9)
    }
    #[doc = "Bit 10 - bit control of ddr1_io_ctrl_oe_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_io_ctrl_oe_polarity(&mut self) -> Ddr1IoCtrlOePolarityW<GrfDdrc1Con0Spec> {
        Ddr1IoCtrlOePolarityW::new(self, 10)
    }
    #[doc = "Bit 11 - bit control of ddr1_dram_clk_enable_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_dram_clk_enable_polarity(
        &mut self,
    ) -> Ddr1DramClkEnablePolarityW<GrfDdrc1Con0Spec> {
        Ddr1DramClkEnablePolarityW::new(self, 11)
    }
    #[doc = "Bit 12 - bit control of ddr1_oe_polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_oe_polarity(&mut self) -> Ddr1OePolarityW<GrfDdrc1Con0Spec> {
        Ddr1OePolarityW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfDdrc1Con0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "ddrc1 control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_ddrc1_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_ddrc1_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfDdrc1Con0Spec;
impl crate::RegisterSpec for GrfDdrc1Con0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_ddrc1_con0::R`](R) reader structure"]
impl crate::Readable for GrfDdrc1Con0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_ddrc1_con0::W`](W) writer structure"]
impl crate::Writable for GrfDdrc1Con0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_DDRC1_CON0 to value 0x1f81"]
impl crate::Resettable for GrfDdrc1Con0Spec {
    const RESET_VALUE: u32 = 0x1f81;
}
