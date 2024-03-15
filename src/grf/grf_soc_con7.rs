#[doc = "Register `GRF_SOC_CON7` reader"]
pub type R = crate::R<GrfSocCon7Spec>;
#[doc = "Register `GRF_SOC_CON7` writer"]
pub type W = crate::W<GrfSocCon7Spec>;
#[doc = "UART polarity selection for cts port Every bit for one UART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GrfUartCtsSel {
    #[doc = "1: keep the cts_n value from IO"]
    B1 = 1,
    #[doc = "0: keep the cts_n value from IO"]
    B0 = 0,
}
impl From<GrfUartCtsSel> for u8 {
    #[inline(always)]
    fn from(variant: GrfUartCtsSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GrfUartCtsSel {
    type Ux = u8;
}
#[doc = "Field `GRF_UART_CTS_SEL` reader - UART polarity selection for cts port Every bit for one UART."]
pub type GrfUartCtsSelR = crate::FieldReader<GrfUartCtsSel>;
impl GrfUartCtsSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GrfUartCtsSel> {
        match self.bits {
            1 => Some(GrfUartCtsSel::B1),
            0 => Some(GrfUartCtsSel::B0),
            _ => None,
        }
    }
    #[doc = "keep the cts_n value from IO"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GrfUartCtsSel::B1
    }
    #[doc = "keep the cts_n value from IO"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GrfUartCtsSel::B0
    }
}
#[doc = "Field `GRF_UART_CTS_SEL` writer - UART polarity selection for cts port Every bit for one UART."]
pub type GrfUartCtsSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, GrfUartCtsSel>;
impl<'a, REG> GrfUartCtsSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "keep the cts_n value from IO"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GrfUartCtsSel::B1)
    }
    #[doc = "keep the cts_n value from IO"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GrfUartCtsSel::B0)
    }
}
#[doc = "uart_rts_sel bit control UART polarity selection for rts port Every bit for one UART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GrfUartRtsSel {
    #[doc = "1: keep the rts_n value from UART module output"]
    B1 = 1,
    #[doc = "0: keep the rts_n value from UART module output"]
    B0 = 0,
}
impl From<GrfUartRtsSel> for u8 {
    #[inline(always)]
    fn from(variant: GrfUartRtsSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GrfUartRtsSel {
    type Ux = u8;
}
#[doc = "Field `GRF_UART_RTS_SEL` reader - uart_rts_sel bit control UART polarity selection for rts port Every bit for one UART."]
pub type GrfUartRtsSelR = crate::FieldReader<GrfUartRtsSel>;
impl GrfUartRtsSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GrfUartRtsSel> {
        match self.bits {
            1 => Some(GrfUartRtsSel::B1),
            0 => Some(GrfUartRtsSel::B0),
            _ => None,
        }
    }
    #[doc = "keep the rts_n value from UART module output"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GrfUartRtsSel::B1
    }
    #[doc = "keep the rts_n value from UART module output"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GrfUartRtsSel::B0
    }
}
#[doc = "Field `GRF_UART_RTS_SEL` writer - uart_rts_sel bit control UART polarity selection for rts port Every bit for one UART."]
pub type GrfUartRtsSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, GrfUartRtsSel>;
impl<'a, REG> GrfUartRtsSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "keep the rts_n value from UART module output"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GrfUartRtsSel::B1)
    }
    #[doc = "keep the rts_n value from UART module output"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GrfUartRtsSel::B0)
    }
}
#[doc = "Field `GRF_UART_DBG_SEL` reader - grf_uart_dbg_sel"]
pub type GrfUartDbgSelR = crate::FieldReader;
#[doc = "Field `GRF_UART_DBG_SEL` writer - grf_uart_dbg_sel"]
pub type GrfUartDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GRF_CON_FORCE_JTAG` reader - grf_con_force_jtag"]
pub type GrfConForceJtagR = crate::BitReader;
#[doc = "Field `GRF_CON_FORCE_JTAG` writer - grf_con_force_jtag"]
pub type GrfConForceJtagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIE_CLKREQ_SEL` reader - pcie_clkreq_sel port control"]
pub type PcieClkreqSelR = crate::BitReader;
#[doc = "Field `PCIE_CLKREQ_SEL` writer - pcie_clkreq_sel port control"]
pub type PcieClkreqSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "gic_awuser mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GicAwuserMode {
    #[doc = "1: user mode"]
    B1 = 1,
    #[doc = "0: user mode"]
    B0 = 0,
}
impl From<GicAwuserMode> for bool {
    #[inline(always)]
    fn from(variant: GicAwuserMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIC_AWUSER_MODE` reader - gic_awuser mode select"]
pub type GicAwuserModeR = crate::BitReader<GicAwuserMode>;
impl GicAwuserModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GicAwuserMode {
        match self.bits {
            true => GicAwuserMode::B1,
            false => GicAwuserMode::B0,
        }
    }
    #[doc = "user mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GicAwuserMode::B1
    }
    #[doc = "user mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GicAwuserMode::B0
    }
}
#[doc = "Field `GIC_AWUSER_MODE` writer - gic_awuser mode select"]
pub type GicAwuserModeW<'a, REG> = crate::BitWriter<'a, REG, GicAwuserMode>;
impl<'a, REG> GicAwuserModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "user mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GicAwuserMode::B1)
    }
    #[doc = "user mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GicAwuserMode::B0)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - UART polarity selection for cts port Every bit for one UART."]
    #[inline(always)]
    pub fn grf_uart_cts_sel(&self) -> GrfUartCtsSelR {
        GrfUartCtsSelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - uart_rts_sel bit control UART polarity selection for rts port Every bit for one UART."]
    #[inline(always)]
    pub fn grf_uart_rts_sel(&self) -> GrfUartRtsSelR {
        GrfUartRtsSelR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - grf_uart_dbg_sel"]
    #[inline(always)]
    pub fn grf_uart_dbg_sel(&self) -> GrfUartDbgSelR {
        GrfUartDbgSelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - grf_con_force_jtag"]
    #[inline(always)]
    pub fn grf_con_force_jtag(&self) -> GrfConForceJtagR {
        GrfConForceJtagR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - pcie_clkreq_sel port control"]
    #[inline(always)]
    pub fn pcie_clkreq_sel(&self) -> PcieClkreqSelR {
        PcieClkreqSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - gic_awuser mode select"]
    #[inline(always)]
    pub fn gic_awuser_mode(&self) -> GicAwuserModeR {
        GicAwuserModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - UART polarity selection for cts port Every bit for one UART."]
    #[inline(always)]
    #[must_use]
    pub fn grf_uart_cts_sel(&mut self) -> GrfUartCtsSelW<GrfSocCon7Spec> {
        GrfUartCtsSelW::new(self, 0)
    }
    #[doc = "Bits 5:9 - uart_rts_sel bit control UART polarity selection for rts port Every bit for one UART."]
    #[inline(always)]
    #[must_use]
    pub fn grf_uart_rts_sel(&mut self) -> GrfUartRtsSelW<GrfSocCon7Spec> {
        GrfUartRtsSelW::new(self, 5)
    }
    #[doc = "Bits 10:11 - grf_uart_dbg_sel"]
    #[inline(always)]
    #[must_use]
    pub fn grf_uart_dbg_sel(&mut self) -> GrfUartDbgSelW<GrfSocCon7Spec> {
        GrfUartDbgSelW::new(self, 10)
    }
    #[doc = "Bit 12 - grf_con_force_jtag"]
    #[inline(always)]
    #[must_use]
    pub fn grf_con_force_jtag(&mut self) -> GrfConForceJtagW<GrfSocCon7Spec> {
        GrfConForceJtagW::new(self, 12)
    }
    #[doc = "Bit 14 - pcie_clkreq_sel port control"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_clkreq_sel(&mut self) -> PcieClkreqSelW<GrfSocCon7Spec> {
        PcieClkreqSelW::new(self, 14)
    }
    #[doc = "Bit 15 - gic_awuser mode select"]
    #[inline(always)]
    #[must_use]
    pub fn gic_awuser_mode(&mut self) -> GicAwuserModeW<GrfSocCon7Spec> {
        GicAwuserModeW::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon7Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon7Spec;
impl crate::RegisterSpec for GrfSocCon7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con7::R`](R) reader structure"]
impl crate::Readable for GrfSocCon7Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con7::W`](W) writer structure"]
impl crate::Writable for GrfSocCon7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON7 to value 0x1000"]
impl crate::Resettable for GrfSocCon7Spec {
    const RESET_VALUE: u32 = 0x1000;
}
