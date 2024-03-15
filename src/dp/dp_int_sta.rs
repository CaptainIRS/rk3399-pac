#[doc = "Register `DP_INT_STA` reader"]
pub type R = crate::R<DpIntStaSpec>;
#[doc = "Register `DP_INT_STA` writer"]
pub type W = crate::W<DpIntStaSpec>;
#[doc = "AUX channel access error interrupt:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxErr {
    #[doc = "1: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B1 = 1,
    #[doc = "0: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B0 = 0,
}
impl From<AuxErr> for bool {
    #[inline(always)]
    fn from(variant: AuxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_ERR` reader - AUX channel access error interrupt:"]
pub type AuxErrR = crate::BitReader<AuxErr>;
impl AuxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxErr {
        match self.bits {
            true => AuxErr::B1,
            false => AuxErr::B0,
        }
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AuxErr::B1
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AuxErr::B0
    }
}
#[doc = "Field `AUX_ERR` writer - AUX channel access error interrupt:"]
pub type AuxErrW<'a, REG> = crate::BitWriter<'a, REG, AuxErr>;
impl<'a, REG> AuxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AuxErr::B1)
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AuxErr::B0)
    }
}
#[doc = "AUX channel command reply is received:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RplyReceiv {
    #[doc = "1: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B1 = 1,
    #[doc = "0: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B0 = 0,
}
impl From<RplyReceiv> for bool {
    #[inline(always)]
    fn from(variant: RplyReceiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPLY_RECEIV` reader - AUX channel command reply is received:"]
pub type RplyReceivR = crate::BitReader<RplyReceiv>;
impl RplyReceivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RplyReceiv {
        match self.bits {
            true => RplyReceiv::B1,
            false => RplyReceiv::B0,
        }
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RplyReceiv::B1
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RplyReceiv::B0
    }
}
#[doc = "Field `RPLY_RECEIV` writer - AUX channel command reply is received:"]
pub type RplyReceivW<'a, REG> = crate::BitWriter1C<'a, REG, RplyReceiv>;
impl<'a, REG> RplyReceivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RplyReceiv::B1)
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RplyReceiv::B0)
    }
}
#[doc = "Link lost interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkLost {
    #[doc = "1: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B1 = 1,
    #[doc = "0: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B0 = 0,
}
impl From<LinkLost> for bool {
    #[inline(always)]
    fn from(variant: LinkLost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINK_LOST` reader - Link lost interrupt"]
pub type LinkLostR = crate::BitReader<LinkLost>;
impl LinkLostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinkLost {
        match self.bits {
            true => LinkLost::B1,
            false => LinkLost::B0,
        }
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LinkLost::B1
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LinkLost::B0
    }
}
#[doc = "Field `LINK_LOST` writer - Link lost interrupt"]
pub type LinkLostW<'a, REG> = crate::BitWriter<'a, REG, LinkLost>;
impl<'a, REG> LinkLostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LinkLost::B1)
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LinkLost::B0)
    }
}
#[doc = "Sink lost interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SinkLost {
    #[doc = "1: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B1 = 1,
    #[doc = "0: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B0 = 0,
}
impl From<SinkLost> for bool {
    #[inline(always)]
    fn from(variant: SinkLost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINK_LOST` reader - Sink lost interrupt"]
pub type SinkLostR = crate::BitReader<SinkLost>;
impl SinkLostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SinkLost {
        match self.bits {
            true => SinkLost::B1,
            false => SinkLost::B0,
        }
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SinkLost::B1
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SinkLost::B0
    }
}
#[doc = "Field `SINK_LOST` writer - Sink lost interrupt"]
pub type SinkLostW<'a, REG> = crate::BitWriter<'a, REG, SinkLost>;
impl<'a, REG> SinkLostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SinkLost::B1)
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SinkLost::B0)
    }
}
#[doc = "Training FSM module finish link training procedure:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwTrainingFinish {
    #[doc = "1: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B1 = 1,
    #[doc = "0: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B0 = 0,
}
impl From<HwTrainingFinish> for bool {
    #[inline(always)]
    fn from(variant: HwTrainingFinish) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HW_TRAINING_FINISH` reader - Training FSM module finish link training procedure:"]
pub type HwTrainingFinishR = crate::BitReader<HwTrainingFinish>;
impl HwTrainingFinishR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwTrainingFinish {
        match self.bits {
            true => HwTrainingFinish::B1,
            false => HwTrainingFinish::B0,
        }
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwTrainingFinish::B1
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwTrainingFinish::B0
    }
}
#[doc = "Field `HW_TRAINING_FINISH` writer - Training FSM module finish link training procedure:"]
pub type HwTrainingFinishW<'a, REG> = crate::BitWriter<'a, REG, HwTrainingFinish>;
impl<'a, REG> HwTrainingFinishW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwTrainingFinish::B1)
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwTrainingFinish::B0)
    }
}
#[doc = "IRQ (HPD de-asserted less than 2ms) detect interrupt:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntHpd {
    #[doc = "1: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B1 = 1,
    #[doc = "0: Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    B0 = 0,
}
impl From<IntHpd> for bool {
    #[inline(always)]
    fn from(variant: IntHpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_HPD` reader - IRQ (HPD de-asserted less than 2ms) detect interrupt:"]
pub type IntHpdR = crate::BitReader<IntHpd>;
impl IntHpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntHpd {
        match self.bits {
            true => IntHpd::B1,
            false => IntHpd::B0,
        }
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntHpd::B1
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntHpd::B0
    }
}
#[doc = "Field `INT_HPD` writer - IRQ (HPD de-asserted less than 2ms) detect interrupt:"]
pub type IntHpdW<'a, REG> = crate::BitWriter<'a, REG, IntHpd>;
impl<'a, REG> IntHpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntHpd::B1)
    }
    #[doc = "Not interrupt occurred Write 1 to this bit to clear this interrupt source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntHpd::B0)
    }
}
impl R {
    #[doc = "Bit 0 - AUX channel access error interrupt:"]
    #[inline(always)]
    pub fn aux_err(&self) -> AuxErrR {
        AuxErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AUX channel command reply is received:"]
    #[inline(always)]
    pub fn rply_receiv(&self) -> RplyReceivR {
        RplyReceivR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Link lost interrupt"]
    #[inline(always)]
    pub fn link_lost(&self) -> LinkLostR {
        LinkLostR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sink lost interrupt"]
    #[inline(always)]
    pub fn sink_lost(&self) -> SinkLostR {
        SinkLostR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Training FSM module finish link training procedure:"]
    #[inline(always)]
    pub fn hw_training_finish(&self) -> HwTrainingFinishR {
        HwTrainingFinishR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ (HPD de-asserted less than 2ms) detect interrupt:"]
    #[inline(always)]
    pub fn int_hpd(&self) -> IntHpdR {
        IntHpdR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AUX channel access error interrupt:"]
    #[inline(always)]
    #[must_use]
    pub fn aux_err(&mut self) -> AuxErrW<DpIntStaSpec> {
        AuxErrW::new(self, 0)
    }
    #[doc = "Bit 1 - AUX channel command reply is received:"]
    #[inline(always)]
    #[must_use]
    pub fn rply_receiv(&mut self) -> RplyReceivW<DpIntStaSpec> {
        RplyReceivW::new(self, 1)
    }
    #[doc = "Bit 2 - Link lost interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn link_lost(&mut self) -> LinkLostW<DpIntStaSpec> {
        LinkLostW::new(self, 2)
    }
    #[doc = "Bit 3 - Sink lost interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sink_lost(&mut self) -> SinkLostW<DpIntStaSpec> {
        SinkLostW::new(self, 3)
    }
    #[doc = "Bit 5 - Training FSM module finish link training procedure:"]
    #[inline(always)]
    #[must_use]
    pub fn hw_training_finish(&mut self) -> HwTrainingFinishW<DpIntStaSpec> {
        HwTrainingFinishW::new(self, 5)
    }
    #[doc = "Bit 6 - IRQ (HPD de-asserted less than 2ms) detect interrupt:"]
    #[inline(always)]
    #[must_use]
    pub fn int_hpd(&mut self) -> IntHpdW<DpIntStaSpec> {
        IntHpdW::new(self, 6)
    }
}
#[doc = "DisplayPort Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_int_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_int_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpIntStaSpec;
impl crate::RegisterSpec for DpIntStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_int_sta::R`](R) reader structure"]
impl crate::Readable for DpIntStaSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_int_sta::W`](W) writer structure"]
impl crate::Writable for DpIntStaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x02;
}
#[doc = "`reset()` method sets DP_INT_STA to value 0"]
impl crate::Resettable for DpIntStaSpec {
    const RESET_VALUE: u32 = 0;
}
