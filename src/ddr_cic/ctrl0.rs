#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "Frequency change request\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChgReq {
    #[doc = "0: not request"]
    B0 = 0,
    #[doc = "1: request"]
    B1 = 1,
}
impl From<ChgReq> for bool {
    #[inline(always)]
    fn from(variant: ChgReq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHG_REQ` reader - Frequency change request"]
pub type ChgReqR = crate::BitReader<ChgReq>;
impl ChgReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChgReq {
        match self.bits {
            false => ChgReq::B0,
            true => ChgReq::B1,
        }
    }
    #[doc = "not request"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ChgReq::B0
    }
    #[doc = "request"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ChgReq::B1
    }
}
#[doc = "Field `CHG_REQ` writer - Frequency change request"]
pub type ChgReqW<'a, REG> = crate::BitWriter<'a, REG, ChgReq>;
impl<'a, REG> ChgReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not request"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ChgReq::B0)
    }
    #[doc = "request"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ChgReq::B1)
    }
}
#[doc = "Frequency change finish\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChgFreqFinish {
    #[doc = "0: not finish"]
    B0 = 0,
    #[doc = "1: finish"]
    B1 = 1,
}
impl From<ChgFreqFinish> for bool {
    #[inline(always)]
    fn from(variant: ChgFreqFinish) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHG_FREQ_FINISH` reader - Frequency change finish"]
pub type ChgFreqFinishR = crate::BitReader<ChgFreqFinish>;
impl ChgFreqFinishR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChgFreqFinish {
        match self.bits {
            false => ChgFreqFinish::B0,
            true => ChgFreqFinish::B1,
        }
    }
    #[doc = "not finish"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ChgFreqFinish::B0
    }
    #[doc = "finish"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ChgFreqFinish::B1
    }
}
#[doc = "Field `CHG_FREQ_FINISH` writer - Frequency change finish"]
pub type ChgFreqFinishW<'a, REG> = crate::BitWriter<'a, REG, ChgFreqFinish>;
impl<'a, REG> ChgFreqFinishW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not finish"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ChgFreqFinish::B0)
    }
    #[doc = "finish"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ChgFreqFinish::B1)
    }
}
#[doc = "Field `FAIL_CONT_EN` reader - When frequency change fail, whether continue to enable change."]
pub type FailContEnR = crate::BitReader;
#[doc = "Field `FAIL_CONT_EN` writer - When frequency change fail, whether continue to enable change."]
pub type FailContEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHG_FC_REG_COPY` reader - Select the copy of timing parameters that will be used after\n\nfrequency change."]
pub type ChgFcRegCopyR = crate::FieldReader;
#[doc = "Field `CHG_FC_REG_COPY` writer - Select the copy of timing parameters that will be used after\n\nfrequency change."]
pub type ChgFcRegCopyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR0_FREQ_CHANGE_ACK` reader - Channel 0 DDR PHY frequency change acknowledge"]
pub type Ddr0FreqChangeAckR = crate::BitReader;
#[doc = "Field `DDR0_FREQ_CHANGE_ACK` writer - Channel 0 DDR PHY frequency change acknowledge"]
pub type Ddr0FreqChangeAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_FREQ_CHANGE_ACK` reader - Channel 1 DDR PHY frequency change acknowledge"]
pub type Ddr1FreqChangeAckR = crate::BitReader;
#[doc = "Field `DDR1_FREQ_CHANGE_ACK` writer - Channel 1 DDR PHY frequency change acknowledge"]
pub type Ddr1FreqChangeAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR0_CNTRL_FREQ_CHANGE_ACK` reader - Channel 0 DDR controller frequency change acknowledge"]
pub type Ddr0CntrlFreqChangeAckR = crate::BitReader;
#[doc = "Field `DDR0_CNTRL_FREQ_CHANGE_ACK` writer - Channel 0 DDR controller frequency change acknowledge"]
pub type Ddr0CntrlFreqChangeAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR1_CNTRL_FREQ_CHANGE_ACK` reader - Channel 1 DDR controller frequency change acknowledge"]
pub type Ddr1CntrlFreqChangeAckR = crate::BitReader;
#[doc = "Field `DDR1_CNTRL_FREQ_CHANGE_ACK` writer - Channel 1 DDR controller frequency change acknowledge"]
pub type Ddr1CntrlFreqChangeAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Frequency change request"]
    #[inline(always)]
    pub fn chg_req(&self) -> ChgReqR {
        ChgReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frequency change finish"]
    #[inline(always)]
    pub fn chg_freq_finish(&self) -> ChgFreqFinishR {
        ChgFreqFinishR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When frequency change fail, whether continue to enable change."]
    #[inline(always)]
    pub fn fail_cont_en(&self) -> FailContEnR {
        FailContEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Select the copy of timing parameters that will be used after\n\nfrequency change."]
    #[inline(always)]
    pub fn chg_fc_reg_copy(&self) -> ChgFcRegCopyR {
        ChgFcRegCopyR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Channel 0 DDR PHY frequency change acknowledge"]
    #[inline(always)]
    pub fn ddr0_freq_change_ack(&self) -> Ddr0FreqChangeAckR {
        Ddr0FreqChangeAckR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 DDR PHY frequency change acknowledge"]
    #[inline(always)]
    pub fn ddr1_freq_change_ack(&self) -> Ddr1FreqChangeAckR {
        Ddr1FreqChangeAckR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 0 DDR controller frequency change acknowledge"]
    #[inline(always)]
    pub fn ddr0_cntrl_freq_change_ack(&self) -> Ddr0CntrlFreqChangeAckR {
        Ddr0CntrlFreqChangeAckR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 1 DDR controller frequency change acknowledge"]
    #[inline(always)]
    pub fn ddr1_cntrl_freq_change_ack(&self) -> Ddr1CntrlFreqChangeAckR {
        Ddr1CntrlFreqChangeAckR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency change request"]
    #[inline(always)]
    #[must_use]
    pub fn chg_req(&mut self) -> ChgReqW<Ctrl0Spec> {
        ChgReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Frequency change finish"]
    #[inline(always)]
    #[must_use]
    pub fn chg_freq_finish(&mut self) -> ChgFreqFinishW<Ctrl0Spec> {
        ChgFreqFinishW::new(self, 1)
    }
    #[doc = "Bit 2 - When frequency change fail, whether continue to enable change."]
    #[inline(always)]
    #[must_use]
    pub fn fail_cont_en(&mut self) -> FailContEnW<Ctrl0Spec> {
        FailContEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Select the copy of timing parameters that will be used after\n\nfrequency change."]
    #[inline(always)]
    #[must_use]
    pub fn chg_fc_reg_copy(&mut self) -> ChgFcRegCopyW<Ctrl0Spec> {
        ChgFcRegCopyW::new(self, 4)
    }
    #[doc = "Bit 8 - Channel 0 DDR PHY frequency change acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_freq_change_ack(&mut self) -> Ddr0FreqChangeAckW<Ctrl0Spec> {
        Ddr0FreqChangeAckW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 DDR PHY frequency change acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_freq_change_ack(&mut self) -> Ddr1FreqChangeAckW<Ctrl0Spec> {
        Ddr1FreqChangeAckW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 0 DDR controller frequency change acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ddr0_cntrl_freq_change_ack(&mut self) -> Ddr0CntrlFreqChangeAckW<Ctrl0Spec> {
        Ddr0CntrlFreqChangeAckW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 1 DDR controller frequency change acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ddr1_cntrl_freq_change_ack(&mut self) -> Ddr1CntrlFreqChangeAckW<Ctrl0Spec> {
        Ddr1CntrlFreqChangeAckW::new(self, 11)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by software.\n\nWhen bit 16=0, bit 0 cannot be written by software;\n\nWhen bit 17=1, bit 1 can be written by software.\n\nWhen bit 17=0, bit 1 cannot be written by software;\n\n......\n\nWhen bit 31=1, bit 15 can be written by software.\n\nWhen bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Ctrl0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "DDR Controller LP Interface Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for Ctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
