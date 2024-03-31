#[doc = "Register `DEBUG` reader"]
pub type R = crate::R<DebugSpec>;
#[doc = "Register `DEBUG` writer"]
pub type W = crate::W<DebugSpec>;
#[doc = "Field `RDB` reader - When high, it indicates that the MAC GMII/MII receive protocol\n\nengine is actively receiving data and not in IDLE state."]
pub type RdbR = crate::BitReader;
#[doc = "Field `RDB` writer - When high, it indicates that the MAC GMII/MII receive protocol\n\nengine is actively receiving data and not in IDLE state."]
pub type RdbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACT` reader - When high, it indicates the active state of the small FIFO Read\n\nand Write controllers respectively of the MAC receive Frame\n\nController module"]
pub type ActR = crate::FieldReader;
#[doc = "Field `ACT` writer - When high, it indicates the active state of the small FIFO Read\n\nand Write controllers respectively of the MAC receive Frame\n\nController module"]
pub type ActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RFIFOWR` reader - When high, it indicates that the MTL RxFIFO Write Controller is\n\nactive and transferring a received frame to the FIFO."]
pub type RfifowrR = crate::BitReader;
#[doc = "Field `RFIFOWR` writer - When high, it indicates that the MTL RxFIFO Write Controller is\n\nactive and transferring a received frame to the FIFO."]
pub type RfifowrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "It gives the state of the RxFIFO read Controller:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfiford {
    #[doc = "0: IDLE state"]
    B00 = 0,
    #[doc = "1: Reading frame data"]
    B01 = 1,
    #[doc = "2: Reading frame status (or time-stamp)"]
    B10 = 2,
    #[doc = "3: Flushing the frame data and Status"]
    B11 = 3,
}
impl From<Rfiford> for u8 {
    #[inline(always)]
    fn from(variant: Rfiford) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfiford {
    type Ux = u8;
}
#[doc = "Field `RFIFORD` reader - It gives the state of the RxFIFO read Controller:"]
pub type RfifordR = crate::FieldReader<Rfiford>;
impl RfifordR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfiford {
        match self.bits {
            0 => Rfiford::B00,
            1 => Rfiford::B01,
            2 => Rfiford::B10,
            3 => Rfiford::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rfiford::B00
    }
    #[doc = "Reading frame data"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rfiford::B01
    }
    #[doc = "Reading frame status (or time-stamp)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Rfiford::B10
    }
    #[doc = "Flushing the frame data and Status"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Rfiford::B11
    }
}
#[doc = "Field `RFIFORD` writer - It gives the state of the RxFIFO read Controller:"]
pub type RfifordW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rfiford>;
impl<'a, REG> RfifordW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rfiford::B00)
    }
    #[doc = "Reading frame data"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rfiford::B01)
    }
    #[doc = "Reading frame status (or time-stamp)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Rfiford::B10)
    }
    #[doc = "Flushing the frame data and Status"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Rfiford::B11)
    }
}
#[doc = "This gives the status of the RxFIFO Fill-level:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfifo {
    #[doc = "0: RxFIFO Empty"]
    B00 = 0,
    #[doc = "1: RxFIFO fill-level below flow-control de-activate threshold"]
    B01 = 1,
    #[doc = "2: RxFIFO fill-level above flow-control activate threshold"]
    B10 = 2,
    #[doc = "3: RxFIFO Full"]
    B11 = 3,
}
impl From<Rfifo> for u8 {
    #[inline(always)]
    fn from(variant: Rfifo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfifo {
    type Ux = u8;
}
#[doc = "Field `RFIFO` reader - This gives the status of the RxFIFO Fill-level:"]
pub type RfifoR = crate::FieldReader<Rfifo>;
impl RfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfifo {
        match self.bits {
            0 => Rfifo::B00,
            1 => Rfifo::B01,
            2 => Rfifo::B10,
            3 => Rfifo::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "RxFIFO Empty"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rfifo::B00
    }
    #[doc = "RxFIFO fill-level below flow-control de-activate threshold"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rfifo::B01
    }
    #[doc = "RxFIFO fill-level above flow-control activate threshold"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Rfifo::B10
    }
    #[doc = "RxFIFO Full"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Rfifo::B11
    }
}
#[doc = "Field `RFIFO` writer - This gives the status of the RxFIFO Fill-level:"]
pub type RfifoW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rfifo>;
impl<'a, REG> RfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RxFIFO Empty"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rfifo::B00)
    }
    #[doc = "RxFIFO fill-level below flow-control de-activate threshold"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rfifo::B01)
    }
    #[doc = "RxFIFO fill-level above flow-control activate threshold"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Rfifo::B10)
    }
    #[doc = "RxFIFO Full"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Rfifo::B11)
    }
}
#[doc = "Field `TACT` reader - When high, it indicates that the MAC GMII/MII transmit protocol\n\nengine is actively transmitting data and not in IDLE state."]
pub type TactR = crate::BitReader;
#[doc = "Field `TACT` writer - When high, it indicates that the MAC GMII/MII transmit protocol\n\nengine is actively transmitting data and not in IDLE state."]
pub type TactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This indicates the state of the MAC Transmit Frame Controller\n\nmodule:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsat {
    #[doc = "0: IDLE"]
    B00 = 0,
    #[doc = "1: Waiting for Status of previous frame or IFG/backoff period to be over"]
    B01 = 1,
    #[doc = "2: Generating and transmitting a PAUSE control frame (in full duplex mode)"]
    B10 = 2,
    #[doc = "3: Transferring input frame for transmission"]
    B11 = 3,
}
impl From<Tsat> for u8 {
    #[inline(always)]
    fn from(variant: Tsat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsat {
    type Ux = u8;
}
#[doc = "Field `TSAT` reader - This indicates the state of the MAC Transmit Frame Controller\n\nmodule:"]
pub type TsatR = crate::FieldReader<Tsat>;
impl TsatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsat {
        match self.bits {
            0 => Tsat::B00,
            1 => Tsat::B01,
            2 => Tsat::B10,
            3 => Tsat::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Tsat::B00
    }
    #[doc = "Waiting for Status of previous frame or IFG/backoff period to be over"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Tsat::B01
    }
    #[doc = "Generating and transmitting a PAUSE control frame (in full duplex mode)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Tsat::B10
    }
    #[doc = "Transferring input frame for transmission"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Tsat::B11
    }
}
#[doc = "Field `TSAT` writer - This indicates the state of the MAC Transmit Frame Controller\n\nmodule:"]
pub type TsatW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Tsat>;
impl<'a, REG> TsatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Tsat::B00)
    }
    #[doc = "Waiting for Status of previous frame or IFG/backoff period to be over"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Tsat::B01)
    }
    #[doc = "Generating and transmitting a PAUSE control frame (in full duplex mode)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Tsat::B10)
    }
    #[doc = "Transferring input frame for transmission"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Tsat::B11)
    }
}
#[doc = "Field `PAUSE` reader - When high, it indicates that the MAC transmitter is in PAUSE\n\ncondition (in full-duplex only) and hence will not schedule any\n\nframe for transmission"]
pub type PauseR = crate::BitReader;
#[doc = "Field `PAUSE` writer - When high, it indicates that the MAC transmitter is in PAUSE\n\ncondition (in full-duplex only) and hence will not schedule any\n\nframe for transmission"]
pub type PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This indicates the state of the TxFIFO read Controller:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tfifosta {
    #[doc = "0: IDLE state"]
    B00 = 0,
    #[doc = "1: READ state (transferring data to MAC transmitter)"]
    B01 = 1,
    #[doc = "2: Waiting for TxStatus from MAC transmitter"]
    B10 = 2,
    #[doc = "3: Writing the received TxStatus or flushing the TxFIFO"]
    B11 = 3,
}
impl From<Tfifosta> for u8 {
    #[inline(always)]
    fn from(variant: Tfifosta) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tfifosta {
    type Ux = u8;
}
#[doc = "Field `TFIFOSTA` reader - This indicates the state of the TxFIFO read Controller:"]
pub type TfifostaR = crate::FieldReader<Tfifosta>;
impl TfifostaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfifosta {
        match self.bits {
            0 => Tfifosta::B00,
            1 => Tfifosta::B01,
            2 => Tfifosta::B10,
            3 => Tfifosta::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Tfifosta::B00
    }
    #[doc = "READ state (transferring data to MAC transmitter)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Tfifosta::B01
    }
    #[doc = "Waiting for TxStatus from MAC transmitter"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Tfifosta::B10
    }
    #[doc = "Writing the received TxStatus or flushing the TxFIFO"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Tfifosta::B11
    }
}
#[doc = "Field `TFIFOSTA` writer - This indicates the state of the TxFIFO read Controller:"]
pub type TfifostaW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Tfifosta>;
impl<'a, REG> TfifostaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Tfifosta::B00)
    }
    #[doc = "READ state (transferring data to MAC transmitter)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Tfifosta::B01)
    }
    #[doc = "Waiting for TxStatus from MAC transmitter"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Tfifosta::B10)
    }
    #[doc = "Writing the received TxStatus or flushing the TxFIFO"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Tfifosta::B11)
    }
}
#[doc = "Field `TFIFO1` reader - When high, it indicates that the MTL TxFIFO Write Controller is\n\nactive and transferring data to the TxFIFO."]
pub type Tfifo1R = crate::BitReader;
#[doc = "Field `TFIFO1` writer - When high, it indicates that the MTL TxFIFO Write Controller is\n\nactive and transferring data to the TxFIFO."]
pub type Tfifo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIFO2` reader - When high, it indicates that the MTL TxFIFO is not empty and has\n\nsome data left for transmission."]
pub type Tfifo2R = crate::BitReader;
#[doc = "Field `TFIFO2` writer - When high, it indicates that the MTL TxFIFO is not empty and has\n\nsome data left for transmission."]
pub type Tfifo2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIFO3` reader - When high, it indicates that the MTL TxStatus FIFO is full and\n\nhence the MTL will not be accepting any more frames for\n\ntransmission."]
pub type Tfifo3R = crate::BitReader;
#[doc = "Field `TFIFO3` writer - When high, it indicates that the MTL TxStatus FIFO is full and\n\nhence the MTL will not be accepting any more frames for\n\ntransmission."]
pub type Tfifo3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When high, it indicates that the MAC GMII/MII receive protocol\n\nengine is actively receiving data and not in IDLE state."]
    #[inline(always)]
    pub fn rdb(&self) -> RdbR {
        RdbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - When high, it indicates the active state of the small FIFO Read\n\nand Write controllers respectively of the MAC receive Frame\n\nController module"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - When high, it indicates that the MTL RxFIFO Write Controller is\n\nactive and transferring a received frame to the FIFO."]
    #[inline(always)]
    pub fn rfifowr(&self) -> RfifowrR {
        RfifowrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - It gives the state of the RxFIFO read Controller:"]
    #[inline(always)]
    pub fn rfiford(&self) -> RfifordR {
        RfifordR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - This gives the status of the RxFIFO Fill-level:"]
    #[inline(always)]
    pub fn rfifo(&self) -> RfifoR {
        RfifoR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - When high, it indicates that the MAC GMII/MII transmit protocol\n\nengine is actively transmitting data and not in IDLE state."]
    #[inline(always)]
    pub fn tact(&self) -> TactR {
        TactR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - This indicates the state of the MAC Transmit Frame Controller\n\nmodule:"]
    #[inline(always)]
    pub fn tsat(&self) -> TsatR {
        TsatR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - When high, it indicates that the MAC transmitter is in PAUSE\n\ncondition (in full-duplex only) and hence will not schedule any\n\nframe for transmission"]
    #[inline(always)]
    pub fn pause(&self) -> PauseR {
        PauseR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - This indicates the state of the TxFIFO read Controller:"]
    #[inline(always)]
    pub fn tfifosta(&self) -> TfifostaR {
        TfifostaR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - When high, it indicates that the MTL TxFIFO Write Controller is\n\nactive and transferring data to the TxFIFO."]
    #[inline(always)]
    pub fn tfifo1(&self) -> Tfifo1R {
        Tfifo1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - When high, it indicates that the MTL TxFIFO is not empty and has\n\nsome data left for transmission."]
    #[inline(always)]
    pub fn tfifo2(&self) -> Tfifo2R {
        Tfifo2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When high, it indicates that the MTL TxStatus FIFO is full and\n\nhence the MTL will not be accepting any more frames for\n\ntransmission."]
    #[inline(always)]
    pub fn tfifo3(&self) -> Tfifo3R {
        Tfifo3R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When high, it indicates that the MAC GMII/MII receive protocol\n\nengine is actively receiving data and not in IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn rdb(&mut self) -> RdbW<DebugSpec> {
        RdbW::new(self, 0)
    }
    #[doc = "Bits 1:2 - When high, it indicates the active state of the small FIFO Read\n\nand Write controllers respectively of the MAC receive Frame\n\nController module"]
    #[inline(always)]
    #[must_use]
    pub fn act(&mut self) -> ActW<DebugSpec> {
        ActW::new(self, 1)
    }
    #[doc = "Bit 4 - When high, it indicates that the MTL RxFIFO Write Controller is\n\nactive and transferring a received frame to the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rfifowr(&mut self) -> RfifowrW<DebugSpec> {
        RfifowrW::new(self, 4)
    }
    #[doc = "Bits 5:6 - It gives the state of the RxFIFO read Controller:"]
    #[inline(always)]
    #[must_use]
    pub fn rfiford(&mut self) -> RfifordW<DebugSpec> {
        RfifordW::new(self, 5)
    }
    #[doc = "Bits 8:9 - This gives the status of the RxFIFO Fill-level:"]
    #[inline(always)]
    #[must_use]
    pub fn rfifo(&mut self) -> RfifoW<DebugSpec> {
        RfifoW::new(self, 8)
    }
    #[doc = "Bit 16 - When high, it indicates that the MAC GMII/MII transmit protocol\n\nengine is actively transmitting data and not in IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn tact(&mut self) -> TactW<DebugSpec> {
        TactW::new(self, 16)
    }
    #[doc = "Bits 17:18 - This indicates the state of the MAC Transmit Frame Controller\n\nmodule:"]
    #[inline(always)]
    #[must_use]
    pub fn tsat(&mut self) -> TsatW<DebugSpec> {
        TsatW::new(self, 17)
    }
    #[doc = "Bit 19 - When high, it indicates that the MAC transmitter is in PAUSE\n\ncondition (in full-duplex only) and hence will not schedule any\n\nframe for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PauseW<DebugSpec> {
        PauseW::new(self, 19)
    }
    #[doc = "Bits 20:21 - This indicates the state of the TxFIFO read Controller:"]
    #[inline(always)]
    #[must_use]
    pub fn tfifosta(&mut self) -> TfifostaW<DebugSpec> {
        TfifostaW::new(self, 20)
    }
    #[doc = "Bit 22 - When high, it indicates that the MTL TxFIFO Write Controller is\n\nactive and transferring data to the TxFIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tfifo1(&mut self) -> Tfifo1W<DebugSpec> {
        Tfifo1W::new(self, 22)
    }
    #[doc = "Bit 24 - When high, it indicates that the MTL TxFIFO is not empty and has\n\nsome data left for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn tfifo2(&mut self) -> Tfifo2W<DebugSpec> {
        Tfifo2W::new(self, 24)
    }
    #[doc = "Bit 25 - When high, it indicates that the MTL TxStatus FIFO is full and\n\nhence the MTL will not be accepting any more frames for\n\ntransmission."]
    #[inline(always)]
    #[must_use]
    pub fn tfifo3(&mut self) -> Tfifo3W<DebugSpec> {
        Tfifo3W::new(self, 25)
    }
}
#[doc = "Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugSpec;
impl crate::RegisterSpec for DebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DebugSpec {}
#[doc = "`write(|w| ..)` method takes [`debug::W`](W) writer structure"]
impl crate::Writable for DebugSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG to value 0"]
impl crate::Resettable for DebugSpec {
    const RESET_VALUE: u32 = 0;
}
