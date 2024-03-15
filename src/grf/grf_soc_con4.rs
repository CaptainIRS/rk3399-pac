#[doc = "Register `GRF_SOC_CON4` reader"]
pub type R = crate::R<GrfSocCon4Spec>;
#[doc = "Register `GRF_SOC_CON4` writer"]
pub type W = crate::W<GrfSocCon4Spec>;
#[doc = "Field `ACCHANNELENS0_CCI500` reader - CCI ACCHANNELEN input control. Slave interface supports DVM messages. This is overridden to 0x0 if you set the Control Override Register\\[1\\]."]
pub type Acchannelens0Cci500R = crate::FieldReader;
#[doc = "Field `ACCHANNELENS0_CCI500` writer - CCI ACCHANNELEN input control. Slave interface supports DVM messages. This is overridden to 0x0 if you set the Control Override Register\\[1\\]."]
pub type Acchannelens0Cci500W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACCHANNELENS1_CCI500` reader - CCI ACCHANNELEN input control. Slave interface supports DVM messages. This is overridden to 0x0 if you set the Control Override Register\\[1\\]."]
pub type Acchannelens1Cci500R = crate::FieldReader;
#[doc = "Field `ACCHANNELENS1_CCI500` writer - CCI ACCHANNELEN input control. Slave interface supports DVM messages. This is overridden to 0x0 if you set the Control Override Register\\[1\\]."]
pub type Acchannelens1Cci500W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CCI_ORDERED_WR_OBSV` reader - cci port ORDERED_WRITE_OBSERVATION control"]
pub type CciOrderedWrObsvR = crate::FieldReader;
#[doc = "Field `CCI_ORDERED_WR_OBSV` writer - cci port ORDERED_WRITE_OBSERVATION control"]
pub type CciOrderedWrObsvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CCI_QOSOVERRIDE` reader - cci port QOSOVERRIDE bit control"]
pub type CciQosoverrideR = crate::FieldReader;
#[doc = "Field `CCI_QOSOVERRIDE` writer - cci port QOSOVERRIDE bit control"]
pub type CciQosoverrideW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "cci force wakeup control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CciForceWakeup {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<CciForceWakeup> for bool {
    #[inline(always)]
    fn from(variant: CciForceWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCI_FORCE_WAKEUP` reader - cci force wakeup control"]
pub type CciForceWakeupR = crate::BitReader<CciForceWakeup>;
impl CciForceWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CciForceWakeup {
        match self.bits {
            false => CciForceWakeup::B0,
            true => CciForceWakeup::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CciForceWakeup::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CciForceWakeup::B1
    }
}
#[doc = "Field `CCI_FORCE_WAKEUP` writer - cci force wakeup control"]
pub type CciForceWakeupW<'a, REG> = crate::BitWriter<'a, REG, CciForceWakeup>;
impl<'a, REG> CciForceWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CciForceWakeup::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CciForceWakeup::B1)
    }
}
#[doc = "select ddr debug port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DdrDebugSel {
    #[doc = "0: ddr_dbug_port\\[63:56\\]"]
    D0 = 0,
    #[doc = "1: ddr_dbug_port\\[63:56\\]"]
    D1 = 1,
    #[doc = "2: ddr_dbug_port\\[63:56\\]"]
    D2 = 2,
    #[doc = "3: ddr_dbug_port\\[63:56\\]"]
    D3 = 3,
    #[doc = "4: ddr_dbug_port\\[63:56\\]"]
    D4 = 4,
    #[doc = "5: ddr_dbug_port\\[63:56\\]"]
    D5 = 5,
    #[doc = "6: ddr_dbug_port\\[63:56\\]"]
    D6 = 6,
    #[doc = "7: ddr_dbug_port\\[63:56\\]"]
    D7 = 7,
}
impl From<DdrDebugSel> for u8 {
    #[inline(always)]
    fn from(variant: DdrDebugSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DdrDebugSel {
    type Ux = u8;
}
#[doc = "Field `DDR_DEBUG_SEL` reader - select ddr debug port"]
pub type DdrDebugSelR = crate::FieldReader<DdrDebugSel>;
impl DdrDebugSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DdrDebugSel {
        match self.bits {
            0 => DdrDebugSel::D0,
            1 => DdrDebugSel::D1,
            2 => DdrDebugSel::D2,
            3 => DdrDebugSel::D3,
            4 => DdrDebugSel::D4,
            5 => DdrDebugSel::D5,
            6 => DdrDebugSel::D6,
            7 => DdrDebugSel::D7,
            _ => unreachable!(),
        }
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == DdrDebugSel::D0
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == DdrDebugSel::D1
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == DdrDebugSel::D2
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == DdrDebugSel::D3
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == DdrDebugSel::D4
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == DdrDebugSel::D5
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == DdrDebugSel::D6
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == DdrDebugSel::D7
    }
}
#[doc = "Field `DDR_DEBUG_SEL` writer - select ddr debug port"]
pub type DdrDebugSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DdrDebugSel>;
impl<'a, REG> DdrDebugSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(DdrDebugSel::D0)
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(DdrDebugSel::D1)
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(DdrDebugSel::D2)
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(DdrDebugSel::D3)
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(DdrDebugSel::D4)
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(DdrDebugSel::D5)
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(DdrDebugSel::D6)
    }
    #[doc = "ddr_dbug_port\\[63:56\\]"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(DdrDebugSel::D7)
    }
}
#[doc = "noc_perilp_fwd_gic_rsp_err_stall bit control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpFwdGicPwrdisctargpwrstall {
    #[doc = "0: stall response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpFwdGicPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpFwdGicPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_FWD_GIC_PWRDISCTARGPWRSTALL` reader - noc_perilp_fwd_gic_rsp_err_stall bit control"]
pub type PerilpFwdGicPwrdisctargpwrstallR = crate::BitReader<PerilpFwdGicPwrdisctargpwrstall>;
impl PerilpFwdGicPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpFwdGicPwrdisctargpwrstall {
        match self.bits {
            false => PerilpFwdGicPwrdisctargpwrstall::B0,
            true => PerilpFwdGicPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpFwdGicPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpFwdGicPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILP_FWD_GIC_PWRDISCTARGPWRSTALL` writer - noc_perilp_fwd_gic_rsp_err_stall bit control"]
pub type PerilpFwdGicPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpFwdGicPwrdisctargpwrstall>;
impl<'a, REG> PerilpFwdGicPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdGicPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdGicPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perilp_fwd_sdioaudio_rsp_err_stall bit control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpFwdSdioaudioPwrdisctargpwrstall {
    #[doc = "0: stall response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpFwdSdioaudioPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpFwdSdioaudioPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_FWD_SDIOAUDIO_PWRDISCTARGPWRSTALL` reader - noc_perilp_fwd_sdioaudio_rsp_err_stall bit control"]
pub type PerilpFwdSdioaudioPwrdisctargpwrstallR =
    crate::BitReader<PerilpFwdSdioaudioPwrdisctargpwrstall>;
impl PerilpFwdSdioaudioPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpFwdSdioaudioPwrdisctargpwrstall {
        match self.bits {
            false => PerilpFwdSdioaudioPwrdisctargpwrstall::B0,
            true => PerilpFwdSdioaudioPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpFwdSdioaudioPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpFwdSdioaudioPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILP_FWD_SDIOAUDIO_PWRDISCTARGPWRSTALL` writer - noc_perilp_fwd_sdioaudio_rsp_err_stall bit control"]
pub type PerilpFwdSdioaudioPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpFwdSdioaudioPwrdisctargpwrstall>;
impl<'a, REG> PerilpFwdSdioaudioPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdSdioaudioPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdSdioaudioPwrdisctargpwrstall::B1)
    }
}
#[doc = "noc_perilp_fwd_centerslv_rsp_err_stall bit control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerilpFwdCenterslvPwrdisctargpwrstall {
    #[doc = "0: stall response"]
    B0 = 0,
    #[doc = "1: stall response"]
    B1 = 1,
}
impl From<PerilpFwdCenterslvPwrdisctargpwrstall> for bool {
    #[inline(always)]
    fn from(variant: PerilpFwdCenterslvPwrdisctargpwrstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERILP_FWD_CENTERSLV_PWRDISCTARGPWRSTALL` reader - noc_perilp_fwd_centerslv_rsp_err_stall bit control"]
pub type PerilpFwdCenterslvPwrdisctargpwrstallR =
    crate::BitReader<PerilpFwdCenterslvPwrdisctargpwrstall>;
impl PerilpFwdCenterslvPwrdisctargpwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PerilpFwdCenterslvPwrdisctargpwrstall {
        match self.bits {
            false => PerilpFwdCenterslvPwrdisctargpwrstall::B0,
            true => PerilpFwdCenterslvPwrdisctargpwrstall::B1,
        }
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PerilpFwdCenterslvPwrdisctargpwrstall::B0
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PerilpFwdCenterslvPwrdisctargpwrstall::B1
    }
}
#[doc = "Field `PERILP_FWD_CENTERSLV_PWRDISCTARGPWRSTALL` writer - noc_perilp_fwd_centerslv_rsp_err_stall bit control"]
pub type PerilpFwdCenterslvPwrdisctargpwrstallW<'a, REG> =
    crate::BitWriter<'a, REG, PerilpFwdCenterslvPwrdisctargpwrstall>;
impl<'a, REG> PerilpFwdCenterslvPwrdisctargpwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdCenterslvPwrdisctargpwrstall::B0)
    }
    #[doc = "stall response"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PerilpFwdCenterslvPwrdisctargpwrstall::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - CCI ACCHANNELEN input control. Slave interface supports DVM messages. This is overridden to 0x0 if you set the Control Override Register\\[1\\]."]
    #[inline(always)]
    pub fn acchannelens0_cci500(&self) -> Acchannelens0Cci500R {
        Acchannelens0Cci500R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CCI ACCHANNELEN input control. Slave interface supports DVM messages. This is overridden to 0x0 if you set the Control Override Register\\[1\\]."]
    #[inline(always)]
    pub fn acchannelens1_cci500(&self) -> Acchannelens1Cci500R {
        Acchannelens1Cci500R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - cci port ORDERED_WRITE_OBSERVATION control"]
    #[inline(always)]
    pub fn cci_ordered_wr_obsv(&self) -> CciOrderedWrObsvR {
        CciOrderedWrObsvR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - cci port QOSOVERRIDE bit control"]
    #[inline(always)]
    pub fn cci_qosoverride(&self) -> CciQosoverrideR {
        CciQosoverrideR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - cci force wakeup control"]
    #[inline(always)]
    pub fn cci_force_wakeup(&self) -> CciForceWakeupR {
        CciForceWakeupR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - select ddr debug port"]
    #[inline(always)]
    pub fn ddr_debug_sel(&self) -> DdrDebugSelR {
        DdrDebugSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - noc_perilp_fwd_gic_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn perilp_fwd_gic_pwrdisctargpwrstall(&self) -> PerilpFwdGicPwrdisctargpwrstallR {
        PerilpFwdGicPwrdisctargpwrstallR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - noc_perilp_fwd_sdioaudio_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn perilp_fwd_sdioaudio_pwrdisctargpwrstall(
        &self,
    ) -> PerilpFwdSdioaudioPwrdisctargpwrstallR {
        PerilpFwdSdioaudioPwrdisctargpwrstallR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - noc_perilp_fwd_centerslv_rsp_err_stall bit control"]
    #[inline(always)]
    pub fn perilp_fwd_centerslv_pwrdisctargpwrstall(
        &self,
    ) -> PerilpFwdCenterslvPwrdisctargpwrstallR {
        PerilpFwdCenterslvPwrdisctargpwrstallR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - CCI ACCHANNELEN input control. Slave interface supports DVM messages. This is overridden to 0x0 if you set the Control Override Register\\[1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn acchannelens0_cci500(&mut self) -> Acchannelens0Cci500W<GrfSocCon4Spec> {
        Acchannelens0Cci500W::new(self, 0)
    }
    #[doc = "Bits 2:3 - CCI ACCHANNELEN input control. Slave interface supports DVM messages. This is overridden to 0x0 if you set the Control Override Register\\[1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn acchannelens1_cci500(&mut self) -> Acchannelens1Cci500W<GrfSocCon4Spec> {
        Acchannelens1Cci500W::new(self, 2)
    }
    #[doc = "Bits 4:5 - cci port ORDERED_WRITE_OBSERVATION control"]
    #[inline(always)]
    #[must_use]
    pub fn cci_ordered_wr_obsv(&mut self) -> CciOrderedWrObsvW<GrfSocCon4Spec> {
        CciOrderedWrObsvW::new(self, 4)
    }
    #[doc = "Bits 6:7 - cci port QOSOVERRIDE bit control"]
    #[inline(always)]
    #[must_use]
    pub fn cci_qosoverride(&mut self) -> CciQosoverrideW<GrfSocCon4Spec> {
        CciQosoverrideW::new(self, 6)
    }
    #[doc = "Bit 8 - cci force wakeup control"]
    #[inline(always)]
    #[must_use]
    pub fn cci_force_wakeup(&mut self) -> CciForceWakeupW<GrfSocCon4Spec> {
        CciForceWakeupW::new(self, 8)
    }
    #[doc = "Bits 9:11 - select ddr debug port"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_debug_sel(&mut self) -> DdrDebugSelW<GrfSocCon4Spec> {
        DdrDebugSelW::new(self, 9)
    }
    #[doc = "Bit 12 - noc_perilp_fwd_gic_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_fwd_gic_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpFwdGicPwrdisctargpwrstallW<GrfSocCon4Spec> {
        PerilpFwdGicPwrdisctargpwrstallW::new(self, 12)
    }
    #[doc = "Bit 13 - noc_perilp_fwd_sdioaudio_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_fwd_sdioaudio_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpFwdSdioaudioPwrdisctargpwrstallW<GrfSocCon4Spec> {
        PerilpFwdSdioaudioPwrdisctargpwrstallW::new(self, 13)
    }
    #[doc = "Bit 14 - noc_perilp_fwd_centerslv_rsp_err_stall bit control"]
    #[inline(always)]
    #[must_use]
    pub fn perilp_fwd_centerslv_pwrdisctargpwrstall(
        &mut self,
    ) -> PerilpFwdCenterslvPwrdisctargpwrstallW<GrfSocCon4Spec> {
        PerilpFwdCenterslvPwrdisctargpwrstallW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon4Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon4Spec;
impl crate::RegisterSpec for GrfSocCon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con4::R`](R) reader structure"]
impl crate::Readable for GrfSocCon4Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con4::W`](W) writer structure"]
impl crate::Writable for GrfSocCon4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON4 to value 0x010f"]
impl crate::Resettable for GrfSocCon4Spec {
    const RESET_VALUE: u32 = 0x010f;
}
