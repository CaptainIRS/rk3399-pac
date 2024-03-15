#[doc = "Register `INT_STATE_0` reader"]
pub type R = crate::R<IntState0Spec>;
#[doc = "Register `INT_STATE_0` writer"]
pub type W = crate::W<IntState0Spec>;
#[doc = "Field `AUX_RETRY_TIMER` reader - AUX Retry Timer Register"]
pub type AuxRetryTimerR = crate::FieldReader;
#[doc = "Field `AUX_RETRY_TIMER` writer - AUX Retry Timer Register"]
pub type AuxRetryTimerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AUX_TC` reader - AUX TC Register"]
pub type AuxTcR = crate::FieldReader;
#[doc = "Field `AUX_TC` writer - AUX TC Register"]
pub type AuxTcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIST_YCBCR422_CRL` reader - For YCbCr422 BIST control"]
pub type BistYcbcr422CrlR = crate::BitReader;
#[doc = "Field `BIST_YCBCR422_CRL` writer - For YCbCr422 BIST control"]
pub type BistYcbcr422CrlW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `M_VID_DEBUG_EN` reader - Enable M_VID debugging"]
pub type MVidDebugEnR = crate::BitReader;
#[doc = "Field `M_VID_DEBUG_EN` writer - Enable M_VID debugging"]
pub type MVidDebugEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "AUX TX enable when AUX_CH_TEST_MODE = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxChEnTest {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<AuxChEnTest> for bool {
    #[inline(always)]
    fn from(variant: AuxChEnTest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_CH_EN_TEST` reader - AUX TX enable when AUX_CH_TEST_MODE = 1."]
pub type AuxChEnTestR = crate::BitReader<AuxChEnTest>;
impl AuxChEnTestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxChEnTest {
        match self.bits {
            false => AuxChEnTest::B0,
            true => AuxChEnTest::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AuxChEnTest::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AuxChEnTest::B1
    }
}
#[doc = "Field `AUX_CH_EN_TEST` writer - AUX TX enable when AUX_CH_TEST_MODE = 1."]
pub type AuxChEnTestW<'a, REG> = crate::BitWriter1C<'a, REG, AuxChEnTest>;
impl<'a, REG> AuxChEnTestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AuxChEnTest::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AuxChEnTest::B1)
    }
}
#[doc = "Field `AUX_CH_T_TEST` reader - AUX transmitted data when AUX_CH_TEST_MODE = 1"]
pub type AuxChTTestR = crate::BitReader;
#[doc = "Field `AUX_CH_T_TEST` writer - AUX transmitted data when AUX_CH_TEST_MODE = 1"]
pub type AuxChTTestW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxChTestMode {
    #[doc = "1: AUX CH is in normal mode."]
    B1 = 1,
    #[doc = "0: AUX CH is in normal mode."]
    B0 = 0,
}
impl From<AuxChTestMode> for bool {
    #[inline(always)]
    fn from(variant: AuxChTestMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_CH_TEST_MODE` reader - "]
pub type AuxChTestModeR = crate::BitReader<AuxChTestMode>;
impl AuxChTestModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxChTestMode {
        match self.bits {
            true => AuxChTestMode::B1,
            false => AuxChTestMode::B0,
        }
    }
    #[doc = "AUX CH is in normal mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AuxChTestMode::B1
    }
    #[doc = "AUX CH is in normal mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AuxChTestMode::B0
    }
}
#[doc = "Field `AUX_CH_TEST_MODE` writer - "]
pub type AuxChTestModeW<'a, REG> = crate::BitWriter1C<'a, REG, AuxChTestMode>;
impl<'a, REG> AuxChTestModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX CH is in normal mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AuxChTestMode::B1)
    }
    #[doc = "AUX CH is in normal mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AuxChTestMode::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxSend0_1En {
    #[doc = "1: normal AUX data transmitting in AUX CH when AUX_CH_TEST_MODE = 0"]
    B1 = 1,
    #[doc = "0: normal AUX data transmitting in AUX CH when AUX_CH_TEST_MODE = 0"]
    B0 = 0,
}
impl From<AuxSend0_1En> for bool {
    #[inline(always)]
    fn from(variant: AuxSend0_1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_SEND_0_1_EN` reader - "]
pub type AuxSend0_1EnR = crate::BitReader<AuxSend0_1En>;
impl AuxSend0_1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxSend0_1En {
        match self.bits {
            true => AuxSend0_1En::B1,
            false => AuxSend0_1En::B0,
        }
    }
    #[doc = "normal AUX data transmitting in AUX CH when AUX_CH_TEST_MODE = 0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AuxSend0_1En::B1
    }
    #[doc = "normal AUX data transmitting in AUX CH when AUX_CH_TEST_MODE = 0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AuxSend0_1En::B0
    }
}
#[doc = "Field `AUX_SEND_0_1_EN` writer - "]
pub type AuxSend0_1EnW<'a, REG> = crate::BitWriter<'a, REG, AuxSend0_1En>;
impl<'a, REG> AuxSend0_1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal AUX data transmitting in AUX CH when AUX_CH_TEST_MODE = 0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AuxSend0_1En::B1)
    }
    #[doc = "normal AUX data transmitting in AUX CH when AUX_CH_TEST_MODE = 0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AuxSend0_1En::B0)
    }
}
#[doc = "Field `AUX_CH_DATA_IN` reader - AUX received data for debug when AUX_CH_TEST_MODE = 1 and AUX_CH_EN_TEST=0 This bit is read only"]
pub type AuxChDataInR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - AUX Retry Timer Register"]
    #[inline(always)]
    pub fn aux_retry_timer(&self) -> AuxRetryTimerR {
        AuxRetryTimerR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - AUX TC Register"]
    #[inline(always)]
    pub fn aux_tc(&self) -> AuxTcR {
        AuxTcR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - For YCbCr422 BIST control"]
    #[inline(always)]
    pub fn bist_ycbcr422_crl(&self) -> BistYcbcr422CrlR {
        BistYcbcr422CrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable M_VID debugging"]
    #[inline(always)]
    pub fn m_vid_debug_en(&self) -> MVidDebugEnR {
        MVidDebugEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AUX TX enable when AUX_CH_TEST_MODE = 1."]
    #[inline(always)]
    pub fn aux_ch_en_test(&self) -> AuxChEnTestR {
        AuxChEnTestR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AUX transmitted data when AUX_CH_TEST_MODE = 1"]
    #[inline(always)]
    pub fn aux_ch_t_test(&self) -> AuxChTTestR {
        AuxChTTestR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn aux_ch_test_mode(&self) -> AuxChTestModeR {
        AuxChTestModeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn aux_send_0_1_en(&self) -> AuxSend0_1EnR {
        AuxSend0_1EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AUX received data for debug when AUX_CH_TEST_MODE = 1 and AUX_CH_EN_TEST=0 This bit is read only"]
    #[inline(always)]
    pub fn aux_ch_data_in(&self) -> AuxChDataInR {
        AuxChDataInR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AUX Retry Timer Register"]
    #[inline(always)]
    #[must_use]
    pub fn aux_retry_timer(&mut self) -> AuxRetryTimerW<IntState0Spec> {
        AuxRetryTimerW::new(self, 0)
    }
    #[doc = "Bits 3:4 - AUX TC Register"]
    #[inline(always)]
    #[must_use]
    pub fn aux_tc(&mut self) -> AuxTcW<IntState0Spec> {
        AuxTcW::new(self, 3)
    }
    #[doc = "Bit 5 - For YCbCr422 BIST control"]
    #[inline(always)]
    #[must_use]
    pub fn bist_ycbcr422_crl(&mut self) -> BistYcbcr422CrlW<IntState0Spec> {
        BistYcbcr422CrlW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable M_VID debugging"]
    #[inline(always)]
    #[must_use]
    pub fn m_vid_debug_en(&mut self) -> MVidDebugEnW<IntState0Spec> {
        MVidDebugEnW::new(self, 6)
    }
    #[doc = "Bit 7 - AUX TX enable when AUX_CH_TEST_MODE = 1."]
    #[inline(always)]
    #[must_use]
    pub fn aux_ch_en_test(&mut self) -> AuxChEnTestW<IntState0Spec> {
        AuxChEnTestW::new(self, 7)
    }
    #[doc = "Bit 8 - AUX transmitted data when AUX_CH_TEST_MODE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn aux_ch_t_test(&mut self) -> AuxChTTestW<IntState0Spec> {
        AuxChTTestW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn aux_ch_test_mode(&mut self) -> AuxChTestModeW<IntState0Spec> {
        AuxChTestModeW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn aux_send_0_1_en(&mut self) -> AuxSend0_1EnW<IntState0Spec> {
        AuxSend0_1EnW::new(self, 10)
    }
}
#[doc = "Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_state_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_state_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntState0Spec;
impl crate::RegisterSpec for IntState0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_state_0::R`](R) reader structure"]
impl crate::Readable for IntState0Spec {}
#[doc = "`write(|w| ..)` method takes [`int_state_0::W`](W) writer structure"]
impl crate::Writable for IntState0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03b8;
}
#[doc = "`reset()` method sets INT_STATE_0 to value 0x03"]
impl crate::Resettable for IntState0Spec {
    const RESET_VALUE: u32 = 0x03;
}
