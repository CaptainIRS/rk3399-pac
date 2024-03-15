#[doc = "Register `AUX_CH_STA` reader"]
pub type R = crate::R<AuxChStaSpec>;
#[doc = "Register `AUX_CH_STA` writer"]
pub type W = crate::W<AuxChStaSpec>;
#[doc = "This register indicate the AUX channel access status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AuxStatus {
    #[doc = "0: I2C_NACK_ERROR Other: Reserved."]
    D0 = 0,
    #[doc = "1: I2C_NACK_ERROR Other: Reserved."]
    D1 = 1,
    #[doc = "2: I2C_NACK_ERROR Other: Reserved."]
    D2 = 2,
    #[doc = "3: I2C_NACK_ERROR Other: Reserved."]
    D3 = 3,
    #[doc = "4: I2C_NACK_ERROR Other: Reserved."]
    D4 = 4,
    #[doc = "5: I2C_NACK_ERROR Other: Reserved."]
    D5 = 5,
    #[doc = "6: I2C_NACK_ERROR Other: Reserved."]
    D6 = 6,
    #[doc = "7: I2C_NACK_ERROR Other: Reserved."]
    D7 = 7,
    #[doc = "8: I2C_NACK_ERROR Other: Reserved."]
    D8 = 8,
}
impl From<AuxStatus> for u8 {
    #[inline(always)]
    fn from(variant: AuxStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AuxStatus {
    type Ux = u8;
}
#[doc = "Field `AUX_STATUS` reader - This register indicate the AUX channel access status"]
pub type AuxStatusR = crate::FieldReader<AuxStatus>;
impl AuxStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AuxStatus> {
        match self.bits {
            0 => Some(AuxStatus::D0),
            1 => Some(AuxStatus::D1),
            2 => Some(AuxStatus::D2),
            3 => Some(AuxStatus::D3),
            4 => Some(AuxStatus::D4),
            5 => Some(AuxStatus::D5),
            6 => Some(AuxStatus::D6),
            7 => Some(AuxStatus::D7),
            8 => Some(AuxStatus::D8),
            _ => None,
        }
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == AuxStatus::D0
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == AuxStatus::D1
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == AuxStatus::D2
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == AuxStatus::D3
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == AuxStatus::D4
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == AuxStatus::D5
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == AuxStatus::D6
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == AuxStatus::D7
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == AuxStatus::D8
    }
}
#[doc = "Field `AUX_STATUS` writer - This register indicate the AUX channel access status"]
pub type AuxStatusW<'a, REG> = crate::FieldWriter<'a, REG, 4, AuxStatus>;
impl<'a, REG> AuxStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(AuxStatus::D0)
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(AuxStatus::D1)
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(AuxStatus::D2)
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(AuxStatus::D3)
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(AuxStatus::D4)
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(AuxStatus::D5)
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(AuxStatus::D6)
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(AuxStatus::D7)
    }
    #[doc = "I2C_NACK_ERROR Other: Reserved."]
    #[inline(always)]
    pub fn d8(self) -> &'a mut crate::W<REG> {
        self.variant(AuxStatus::D8)
    }
}
#[doc = "AUX channel status bit. If this bit is read as 1, AUX channel access should be halted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxBusy {
    #[doc = "1: AUX CH is idle"]
    B1 = 1,
    #[doc = "0: AUX CH is idle"]
    B0 = 0,
}
impl From<AuxBusy> for bool {
    #[inline(always)]
    fn from(variant: AuxBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_BUSY` reader - AUX channel status bit. If this bit is read as 1, AUX channel access should be halted."]
pub type AuxBusyR = crate::BitReader<AuxBusy>;
impl AuxBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxBusy {
        match self.bits {
            true => AuxBusy::B1,
            false => AuxBusy::B0,
        }
    }
    #[doc = "AUX CH is idle"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AuxBusy::B1
    }
    #[doc = "AUX CH is idle"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AuxBusy::B0
    }
}
#[doc = "Field `AUX_BUSY` writer - AUX channel status bit. If this bit is read as 1, AUX channel access should be halted."]
pub type AuxBusyW<'a, REG> = crate::BitWriter<'a, REG, AuxBusy>;
impl<'a, REG> AuxBusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX CH is idle"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AuxBusy::B1)
    }
    #[doc = "AUX CH is idle"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AuxBusy::B0)
    }
}
impl R {
    #[doc = "Bits 0:3 - This register indicate the AUX channel access status"]
    #[inline(always)]
    pub fn aux_status(&self) -> AuxStatusR {
        AuxStatusR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - AUX channel status bit. If this bit is read as 1, AUX channel access should be halted."]
    #[inline(always)]
    pub fn aux_busy(&self) -> AuxBusyR {
        AuxBusyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - This register indicate the AUX channel access status"]
    #[inline(always)]
    #[must_use]
    pub fn aux_status(&mut self) -> AuxStatusW<AuxChStaSpec> {
        AuxStatusW::new(self, 0)
    }
    #[doc = "Bit 4 - AUX channel status bit. If this bit is read as 1, AUX channel access should be halted."]
    #[inline(always)]
    #[must_use]
    pub fn aux_busy(&mut self) -> AuxBusyW<AuxChStaSpec> {
        AuxBusyW::new(self, 4)
    }
}
#[doc = "AUX Channel Access Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aux_ch_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aux_ch_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxChStaSpec;
impl crate::RegisterSpec for AuxChStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aux_ch_sta::R`](R) reader structure"]
impl crate::Readable for AuxChStaSpec {}
#[doc = "`write(|w| ..)` method takes [`aux_ch_sta::W`](W) writer structure"]
impl crate::Writable for AuxChStaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUX_CH_STA to value 0"]
impl crate::Resettable for AuxChStaSpec {
    const RESET_VALUE: u32 = 0;
}
