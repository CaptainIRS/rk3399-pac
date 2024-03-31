#[doc = "Register `CKR` reader"]
pub type R = crate::R<CkrSpec>;
#[doc = "Register `CKR` writer"]
pub type W = crate::W<CkrSpec>;
#[doc = "Field `TSD` reader - Transmit sclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nTransmit sclk divider=Ftxsclk/Ftxlrck\n\n0~30:reserved\n\n31: 32fs\n\n32: 33fs\n\n33: 34fs\n\n34: 35fs\n\n......\n\nn: (n+1)fs\n\n......\n\n253: 254fs\n\n254: 255fs\n\n255: 256fs"]
pub type TsdR = crate::FieldReader;
#[doc = "Field `TSD` writer - Transmit sclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nTransmit sclk divider=Ftxsclk/Ftxlrck\n\n0~30:reserved\n\n31: 32fs\n\n32: 33fs\n\n33: 34fs\n\n34: 35fs\n\n......\n\nn: (n+1)fs\n\n......\n\n253: 254fs\n\n254: 255fs\n\n255: 256fs"]
pub type TsdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSD` reader - Receive sclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nReceive sclk divider= Fsclk/Frxlrck\n\n0~30:reserved\n\n31: 32fs\n\n32: 33fs\n\n33: 34fs\n\n34: 35fs\n\n......\n\nn: (n+1)fs\n\n......\n\n253: 254fs\n\n254: 255fs\n\n255: 256fs"]
pub type RsdR = crate::FieldReader;
#[doc = "Field `RSD` writer - Receive sclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nReceive sclk divider= Fsclk/Frxlrck\n\n0~30:reserved\n\n31: 32fs\n\n32: 33fs\n\n33: 34fs\n\n34: 35fs\n\n......\n\nn: (n+1)fs\n\n......\n\n253: 254fs\n\n254: 255fs\n\n255: 256fs"]
pub type RsdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MDIV` reader - mclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nSerial Clock Divider = Fmclk / Ftxsclk-1.(mclk frequecy / txsclk\n\nfrequecy-1)\n\n0 :Fmclk=Ftxsclk;\n\n1 :Fmclk=2*Ftxsclk;\n\n2,3 :Fmclk=4*Ftxsclk;\n\n4,5 :Fmclk=6*Ftxsclk;\n\n......\n\n2n,2n+1:Fmclk=(2n+2)*Ftxsclk;\n\n......\n\n60,61:Fmclk=62*Ftxsclk;\n\n62,63:Fmclk=64*Ftxsclk;\n\n......\n\n252,253:Fmclk=254*Ftxsclk;\n\n254,255:Fmclk=256*Ftxsclk;"]
pub type MdivR = crate::FieldReader;
#[doc = "Field `MDIV` writer - mclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nSerial Clock Divider = Fmclk / Ftxsclk-1.(mclk frequecy / txsclk\n\nfrequecy-1)\n\n0 :Fmclk=Ftxsclk;\n\n1 :Fmclk=2*Ftxsclk;\n\n2,3 :Fmclk=4*Ftxsclk;\n\n4,5 :Fmclk=6*Ftxsclk;\n\n......\n\n2n,2n+1:Fmclk=(2n+2)*Ftxsclk;\n\n......\n\n60,61:Fmclk=62*Ftxsclk;\n\n62,63:Fmclk=64*Ftxsclk;\n\n......\n\n252,253:Fmclk=254*Ftxsclk;\n\n254,255:Fmclk=256*Ftxsclk;"]
pub type MdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Transmit lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tlp {
    #[doc = "0: normal polarity (I2S normal: low for left channel, high for right channel I2S left/right just: high for left channel, low for right channel PCM start signal: high valid)"]
    B0 = 0,
    #[doc = "1: oppsite polarity (I2S normal: high for left channel, low for right channel I2S left/right just: low for left channel, high for right channel PCM start signal: low valid)"]
    B1 = 1,
}
impl From<Tlp> for bool {
    #[inline(always)]
    fn from(variant: Tlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TLP` reader - Transmit lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
pub type TlpR = crate::BitReader<Tlp>;
impl TlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tlp {
        match self.bits {
            false => Tlp::B0,
            true => Tlp::B1,
        }
    }
    #[doc = "normal polarity (I2S normal: low for left channel, high for right channel I2S left/right just: high for left channel, low for right channel PCM start signal: high valid)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tlp::B0
    }
    #[doc = "oppsite polarity (I2S normal: high for left channel, low for right channel I2S left/right just: low for left channel, high for right channel PCM start signal: low valid)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tlp::B1
    }
}
#[doc = "Field `TLP` writer - Transmit lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
pub type TlpW<'a, REG> = crate::BitWriter<'a, REG, Tlp>;
impl<'a, REG> TlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal polarity (I2S normal: low for left channel, high for right channel I2S left/right just: high for left channel, low for right channel PCM start signal: high valid)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tlp::B0)
    }
    #[doc = "oppsite polarity (I2S normal: high for left channel, low for right channel I2S left/right just: low for left channel, high for right channel PCM start signal: low valid)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tlp::B1)
    }
}
#[doc = "Receive lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rlp {
    #[doc = "0: normal polarity (I2S normal: low for left channel, high for right channel I2S left/right just: high for left channel, low for right channel PCM start signal: high valid)"]
    B0 = 0,
    #[doc = "1: oppsite polarity (I2S normal: high for left channel, low for right channel I2S left/right just: low for left channel, high for right channel PCM start signal: low valid)"]
    B1 = 1,
}
impl From<Rlp> for bool {
    #[inline(always)]
    fn from(variant: Rlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLP` reader - Receive lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
pub type RlpR = crate::BitReader<Rlp>;
impl RlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rlp {
        match self.bits {
            false => Rlp::B0,
            true => Rlp::B1,
        }
    }
    #[doc = "normal polarity (I2S normal: low for left channel, high for right channel I2S left/right just: high for left channel, low for right channel PCM start signal: high valid)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rlp::B0
    }
    #[doc = "oppsite polarity (I2S normal: high for left channel, low for right channel I2S left/right just: low for left channel, high for right channel PCM start signal: low valid)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rlp::B1
    }
}
#[doc = "Field `RLP` writer - Receive lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
pub type RlpW<'a, REG> = crate::BitWriter<'a, REG, Rlp>;
impl<'a, REG> RlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal polarity (I2S normal: low for left channel, high for right channel I2S left/right just: high for left channel, low for right channel PCM start signal: high valid)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rlp::B0)
    }
    #[doc = "oppsite polarity (I2S normal: high for left channel, low for right channel I2S left/right just: low for left channel, high for right channel PCM start signal: low valid)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rlp::B1)
    }
}
#[doc = "Sclk polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckp {
    #[doc = "0: sample data at posedge sclk and drive data at negedge sclk"]
    B0 = 0,
    #[doc = "1: sample data at negedge sclk and drive data at posedge sclk"]
    B1 = 1,
}
impl From<Ckp> for bool {
    #[inline(always)]
    fn from(variant: Ckp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKP` reader - Sclk polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
pub type CkpR = crate::BitReader<Ckp>;
impl CkpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckp {
        match self.bits {
            false => Ckp::B0,
            true => Ckp::B1,
        }
    }
    #[doc = "sample data at posedge sclk and drive data at negedge sclk"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ckp::B0
    }
    #[doc = "sample data at negedge sclk and drive data at posedge sclk"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ckp::B1
    }
}
#[doc = "Field `CKP` writer - Sclk polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
pub type CkpW<'a, REG> = crate::BitWriter<'a, REG, Ckp>;
impl<'a, REG> CkpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sample data at posedge sclk and drive data at negedge sclk"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckp::B0)
    }
    #[doc = "sample data at negedge sclk and drive data at posedge sclk"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckp::B1)
    }
}
#[doc = "Master/slave mode select\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mss {
    #[doc = "0: master mode(sclk output)"]
    B0 = 0,
    #[doc = "1: slave mode(sclk input)"]
    B1 = 1,
}
impl From<Mss> for bool {
    #[inline(always)]
    fn from(variant: Mss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSS` reader - Master/slave mode select\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
pub type MssR = crate::BitReader<Mss>;
impl MssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mss {
        match self.bits {
            false => Mss::B0,
            true => Mss::B1,
        }
    }
    #[doc = "master mode(sclk output)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mss::B0
    }
    #[doc = "slave mode(sclk input)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mss::B1
    }
}
#[doc = "Field `MSS` writer - Master/slave mode select\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
pub type MssW<'a, REG> = crate::BitWriter<'a, REG, Mss>;
impl<'a, REG> MssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "master mode(sclk output)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mss::B0)
    }
    #[doc = "slave mode(sclk input)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mss::B1)
    }
}
#[doc = "Tx and Rx Common Use\n\n2'b00/2'b11:tx_lrck/rx_lrck are used as synchronous signal for\n\nTX /RX respectively.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trcm {
    #[doc = "1: only tx_lrck is used as synchronous signal for TX and RX."]
    B01 = 1,
    #[doc = "2: only rx_lrck is used as synchronous signal for TX and RX."]
    B10 = 2,
}
impl From<Trcm> for u8 {
    #[inline(always)]
    fn from(variant: Trcm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trcm {
    type Ux = u8;
}
#[doc = "Field `TRCM` reader - Tx and Rx Common Use\n\n2'b00/2'b11:tx_lrck/rx_lrck are used as synchronous signal for\n\nTX /RX respectively."]
pub type TrcmR = crate::FieldReader<Trcm>;
impl TrcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trcm> {
        match self.bits {
            1 => Some(Trcm::B01),
            2 => Some(Trcm::B10),
            _ => None,
        }
    }
    #[doc = "only tx_lrck is used as synchronous signal for TX and RX."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Trcm::B01
    }
    #[doc = "only rx_lrck is used as synchronous signal for TX and RX."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Trcm::B10
    }
}
#[doc = "Field `TRCM` writer - Tx and Rx Common Use\n\n2'b00/2'b11:tx_lrck/rx_lrck are used as synchronous signal for\n\nTX /RX respectively."]
pub type TrcmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trcm>;
impl<'a, REG> TrcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "only tx_lrck is used as synchronous signal for TX and RX."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Trcm::B01)
    }
    #[doc = "only rx_lrck is used as synchronous signal for TX and RX."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Trcm::B10)
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit sclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nTransmit sclk divider=Ftxsclk/Ftxlrck\n\n0~30:reserved\n\n31: 32fs\n\n32: 33fs\n\n33: 34fs\n\n34: 35fs\n\n......\n\nn: (n+1)fs\n\n......\n\n253: 254fs\n\n254: 255fs\n\n255: 256fs"]
    #[inline(always)]
    pub fn tsd(&self) -> TsdR {
        TsdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive sclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nReceive sclk divider= Fsclk/Frxlrck\n\n0~30:reserved\n\n31: 32fs\n\n32: 33fs\n\n33: 34fs\n\n34: 35fs\n\n......\n\nn: (n+1)fs\n\n......\n\n253: 254fs\n\n254: 255fs\n\n255: 256fs"]
    #[inline(always)]
    pub fn rsd(&self) -> RsdR {
        RsdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - mclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nSerial Clock Divider = Fmclk / Ftxsclk-1.(mclk frequecy / txsclk\n\nfrequecy-1)\n\n0 :Fmclk=Ftxsclk;\n\n1 :Fmclk=2*Ftxsclk;\n\n2,3 :Fmclk=4*Ftxsclk;\n\n4,5 :Fmclk=6*Ftxsclk;\n\n......\n\n2n,2n+1:Fmclk=(2n+2)*Ftxsclk;\n\n......\n\n60,61:Fmclk=62*Ftxsclk;\n\n62,63:Fmclk=64*Ftxsclk;\n\n......\n\n252,253:Fmclk=254*Ftxsclk;\n\n254,255:Fmclk=256*Ftxsclk;"]
    #[inline(always)]
    pub fn mdiv(&self) -> MdivR {
        MdivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Transmit lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    pub fn tlp(&self) -> TlpR {
        TlpR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    pub fn rlp(&self) -> RlpR {
        RlpR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Sclk polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    pub fn ckp(&self) -> CkpR {
        CkpR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Master/slave mode select\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    pub fn mss(&self) -> MssR {
        MssR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Tx and Rx Common Use\n\n2'b00/2'b11:tx_lrck/rx_lrck are used as synchronous signal for\n\nTX /RX respectively."]
    #[inline(always)]
    pub fn trcm(&self) -> TrcmR {
        TrcmR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit sclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nTransmit sclk divider=Ftxsclk/Ftxlrck\n\n0~30:reserved\n\n31: 32fs\n\n32: 33fs\n\n33: 34fs\n\n34: 35fs\n\n......\n\nn: (n+1)fs\n\n......\n\n253: 254fs\n\n254: 255fs\n\n255: 256fs"]
    #[inline(always)]
    #[must_use]
    pub fn tsd(&mut self) -> TsdW<CkrSpec> {
        TsdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Receive sclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nReceive sclk divider= Fsclk/Frxlrck\n\n0~30:reserved\n\n31: 32fs\n\n32: 33fs\n\n33: 34fs\n\n34: 35fs\n\n......\n\nn: (n+1)fs\n\n......\n\n253: 254fs\n\n254: 255fs\n\n255: 256fs"]
    #[inline(always)]
    #[must_use]
    pub fn rsd(&mut self) -> RsdW<CkrSpec> {
        RsdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - mclk divider\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)\n\nSerial Clock Divider = Fmclk / Ftxsclk-1.(mclk frequecy / txsclk\n\nfrequecy-1)\n\n0 :Fmclk=Ftxsclk;\n\n1 :Fmclk=2*Ftxsclk;\n\n2,3 :Fmclk=4*Ftxsclk;\n\n4,5 :Fmclk=6*Ftxsclk;\n\n......\n\n2n,2n+1:Fmclk=(2n+2)*Ftxsclk;\n\n......\n\n60,61:Fmclk=62*Ftxsclk;\n\n62,63:Fmclk=64*Ftxsclk;\n\n......\n\n252,253:Fmclk=254*Ftxsclk;\n\n254,255:Fmclk=256*Ftxsclk;"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv(&mut self) -> MdivW<CkrSpec> {
        MdivW::new(self, 16)
    }
    #[doc = "Bit 24 - Transmit lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn tlp(&mut self) -> TlpW<CkrSpec> {
        TlpW::new(self, 24)
    }
    #[doc = "Bit 25 - Receive lrck polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn rlp(&mut self) -> RlpW<CkrSpec> {
        RlpW::new(self, 25)
    }
    #[doc = "Bit 26 - Sclk polarity\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn ckp(&mut self) -> CkpW<CkrSpec> {
        CkpW::new(self, 26)
    }
    #[doc = "Bit 27 - Master/slave mode select\n\n(Can be written only when XFER\\[1\\]
or XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn mss(&mut self) -> MssW<CkrSpec> {
        MssW::new(self, 27)
    }
    #[doc = "Bits 28:29 - Tx and Rx Common Use\n\n2'b00/2'b11:tx_lrck/rx_lrck are used as synchronous signal for\n\nTX /RX respectively."]
    #[inline(always)]
    #[must_use]
    pub fn trcm(&mut self) -> TrcmW<CkrSpec> {
        TrcmW::new(self, 28)
    }
}
#[doc = "clock generation register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkrSpec;
impl crate::RegisterSpec for CkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckr::R`](R) reader structure"]
impl crate::Readable for CkrSpec {}
#[doc = "`write(|w| ..)` method takes [`ckr::W`](W) writer structure"]
impl crate::Writable for CkrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKR to value 0x0007_1f1f"]
impl crate::Resettable for CkrSpec {
    const RESET_VALUE: u32 = 0x0007_1f1f;
}
