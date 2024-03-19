#[doc = "Register `PCIE_CLIENT_MSG_CTRL` reader"]
pub type R = crate::R<PcieClientMsgCtrlSpec>;
#[doc = "Register `PCIE_CLIENT_MSG_CTRL` writer"]
pub type W = crate::W<PcieClientMsgCtrlSpec>;
#[doc = "Message fifo receive enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgFifoEn {
    #[doc = "0: disable message receive"]
    B0 = 0,
    #[doc = "1: enable client message receive"]
    B1 = 1,
}
impl From<MsgFifoEn> for bool {
    #[inline(always)]
    fn from(variant: MsgFifoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSG_FIFO_EN` reader - Message fifo receive enable"]
pub type MsgFifoEnR = crate::BitReader<MsgFifoEn>;
impl MsgFifoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsgFifoEn {
        match self.bits {
            false => MsgFifoEn::B0,
            true => MsgFifoEn::B1,
        }
    }
    #[doc = "disable message receive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MsgFifoEn::B0
    }
    #[doc = "enable client message receive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MsgFifoEn::B1
    }
}
#[doc = "Field `MSG_FIFO_EN` writer - Message fifo receive enable"]
pub type MsgFifoEnW<'a, REG> = crate::BitWriter<'a, REG, MsgFifoEn>;
impl<'a, REG> MsgFifoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable message receive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MsgFifoEn::B0)
    }
    #[doc = "enable client message receive"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MsgFifoEn::B1)
    }
}
#[doc = "Message fifo receive mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgFifoRxMode {
    #[doc = "0: partial mode"]
    B0 = 0,
    #[doc = "1: full mode"]
    B1 = 1,
}
impl From<MsgFifoRxMode> for bool {
    #[inline(always)]
    fn from(variant: MsgFifoRxMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSG_FIFO_RX_MODE` reader - Message fifo receive mode select"]
pub type MsgFifoRxModeR = crate::BitReader<MsgFifoRxMode>;
impl MsgFifoRxModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsgFifoRxMode {
        match self.bits {
            false => MsgFifoRxMode::B0,
            true => MsgFifoRxMode::B1,
        }
    }
    #[doc = "partial mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MsgFifoRxMode::B0
    }
    #[doc = "full mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MsgFifoRxMode::B1
    }
}
#[doc = "Field `MSG_FIFO_RX_MODE` writer - Message fifo receive mode select"]
pub type MsgFifoRxModeW<'a, REG> = crate::BitWriter<'a, REG, MsgFifoRxMode>;
impl<'a, REG> MsgFifoRxModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "partial mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(MsgFifoRxMode::B0)
    }
    #[doc = "full mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(MsgFifoRxMode::B1)
    }
}
#[doc = "Field `ALMFULL_WATER_MARK` reader - Almost full water mark\n\nalmost full water mark configuration"]
pub type AlmfullWaterMarkR = crate::FieldReader;
#[doc = "Field `ALMFULL_WATER_MARK` writer - Almost full water mark\n\nalmost full water mark configuration"]
pub type AlmfullWaterMarkW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Write mask\n\nFor each served bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum WriteMask {
    #[doc = "0: write mask"]
    B0 = 0,
    #[doc = "1: write enable"]
    B1 = 1,
}
impl From<WriteMask> for u16 {
    #[inline(always)]
    fn from(variant: WriteMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteMask {
    type Ux = u16;
}
#[doc = "Field `WRITE_MASK` writer - Write mask\n\nFor each served bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, WriteMask>;
impl<'a, REG> WriteMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "write mask"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B0)
    }
    #[doc = "write enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Message fifo receive enable"]
    #[inline(always)]
    pub fn msg_fifo_en(&self) -> MsgFifoEnR {
        MsgFifoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Message fifo receive mode select"]
    #[inline(always)]
    pub fn msg_fifo_rx_mode(&self) -> MsgFifoRxModeR {
        MsgFifoRxModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Almost full water mark\n\nalmost full water mark configuration"]
    #[inline(always)]
    pub fn almfull_water_mark(&self) -> AlmfullWaterMarkR {
        AlmfullWaterMarkR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Message fifo receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn msg_fifo_en(&mut self) -> MsgFifoEnW<PcieClientMsgCtrlSpec> {
        MsgFifoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Message fifo receive mode select"]
    #[inline(always)]
    #[must_use]
    pub fn msg_fifo_rx_mode(&mut self) -> MsgFifoRxModeW<PcieClientMsgCtrlSpec> {
        MsgFifoRxModeW::new(self, 1)
    }
    #[doc = "Bits 8:12 - Almost full water mark\n\nalmost full water mark configuration"]
    #[inline(always)]
    #[must_use]
    pub fn almfull_water_mark(&mut self) -> AlmfullWaterMarkW<PcieClientMsgCtrlSpec> {
        AlmfullWaterMarkW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Write mask\n\nFor each served bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PcieClientMsgCtrlSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Message receive control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_msg_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientMsgCtrlSpec;
impl crate::RegisterSpec for PcieClientMsgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_msg_ctrl::R`](R) reader structure"]
impl crate::Readable for PcieClientMsgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_msg_ctrl::W`](W) writer structure"]
impl crate::Writable for PcieClientMsgCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_CLIENT_MSG_CTRL to value 0"]
impl crate::Resettable for PcieClientMsgCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
