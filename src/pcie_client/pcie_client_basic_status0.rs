#[doc = "Register `PCIE_CLIENT_BASIC_STATUS0` reader"]
pub type R = crate::R<PcieClientBasicStatus0Spec>;
#[doc = "Optimized buffer flush and fill enable This output reflects the setting of the OBFF Enable field in the Device Control 2 Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ObffEn {
    #[doc = "0: OBFF enabled using WAKE# signaling."]
    B00 = 0,
    #[doc = "1: OBFF enabled using WAKE# signaling."]
    B01 = 1,
    #[doc = "2: OBFF enabled using WAKE# signaling."]
    B10 = 2,
    #[doc = "3: OBFF enabled using WAKE# signaling."]
    B11 = 3,
}
impl From<ObffEn> for u8 {
    #[inline(always)]
    fn from(variant: ObffEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ObffEn {
    type Ux = u8;
}
#[doc = "Field `OBFF_EN` reader - Optimized buffer flush and fill enable This output reflects the setting of the OBFF Enable field in the Device Control 2 Register"]
pub type ObffEnR = crate::FieldReader<ObffEn>;
impl ObffEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ObffEn {
        match self.bits {
            0 => ObffEn::B00,
            1 => ObffEn::B01,
            2 => ObffEn::B10,
            3 => ObffEn::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "OBFF enabled using WAKE# signaling."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ObffEn::B00
    }
    #[doc = "OBFF enabled using WAKE# signaling."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ObffEn::B01
    }
    #[doc = "OBFF enabled using WAKE# signaling."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ObffEn::B10
    }
    #[doc = "OBFF enabled using WAKE# signaling."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ObffEn::B11
    }
}
#[doc = "Latency tolerance reporting mechanism enable The state of this output reflects the setting of the LTR Mechanism Enable bit in the Device Control 2 Register of Physical Function 0. When the core is configured as an Endpoint, client logic uses this output to enable the generation of LTR messages. This output is not to be used when the core is configured as a Root Complex.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LtrEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<LtrEn> for bool {
    #[inline(always)]
    fn from(variant: LtrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTR_EN` reader - Latency tolerance reporting mechanism enable The state of this output reflects the setting of the LTR Mechanism Enable bit in the Device Control 2 Register of Physical Function 0. When the core is configured as an Endpoint, client logic uses this output to enable the generation of LTR messages. This output is not to be used when the core is configured as a Root Complex."]
pub type LtrEnR = crate::BitReader<LtrEn>;
impl LtrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LtrEn {
        match self.bits {
            false => LtrEn::B0,
            true => LtrEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LtrEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LtrEn::B1
    }
}
#[doc = "Read completion boundary status Provides the setting of the Read Completion Boundary (RCB) bit in the Link Control Register of each Physical Function. In the Endpoint mode, bit 0 indicates the RCB for PF 0 and so on. In the RC mode, bit 0 indicates the RCB setting of the Link Control Register of the RC. For each bit, a value of\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RcbSt {
    #[doc = "0: indicates 128 bytes"]
    B0 = 0,
    #[doc = "1: indicates 128 bytes"]
    B1 = 1,
}
impl From<RcbSt> for bool {
    #[inline(always)]
    fn from(variant: RcbSt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB_ST` reader - Read completion boundary status Provides the setting of the Read Completion Boundary (RCB) bit in the Link Control Register of each Physical Function. In the Endpoint mode, bit 0 indicates the RCB for PF 0 and so on. In the RC mode, bit 0 indicates the RCB setting of the Link Control Register of the RC. For each bit, a value of"]
pub type RcbStR = crate::BitReader<RcbSt>;
impl RcbStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RcbSt {
        match self.bits {
            false => RcbSt::B0,
            true => RcbSt::B1,
        }
    }
    #[doc = "indicates 128 bytes"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RcbSt::B0
    }
    #[doc = "indicates 128 bytes"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RcbSt::B1
    }
}
#[doc = "Operation speed after negotiation Current operating speed of the link is as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NegotiatedSpeed {
    #[doc = "0: 5GT/s"]
    B0 = 0,
    #[doc = "1: 5GT/s"]
    B1 = 1,
}
impl From<NegotiatedSpeed> for bool {
    #[inline(always)]
    fn from(variant: NegotiatedSpeed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEGOTIATED_SPEED` reader - Operation speed after negotiation Current operating speed of the link is as follows:"]
pub type NegotiatedSpeedR = crate::BitReader<NegotiatedSpeed>;
impl NegotiatedSpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NegotiatedSpeed {
        match self.bits {
            false => NegotiatedSpeed::B0,
            true => NegotiatedSpeed::B1,
        }
    }
    #[doc = "5GT/s"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NegotiatedSpeed::B0
    }
    #[doc = "5GT/s"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NegotiatedSpeed::B1
    }
}
#[doc = "Negotiated link width Current link width are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NegotiatedLinkWidth {
    #[doc = "2: x1 others: Reserved"]
    B10 = 2,
    #[doc = "1: x1 others: Reserved"]
    B01 = 1,
    #[doc = "0: x1 others: Reserved"]
    B00 = 0,
}
impl From<NegotiatedLinkWidth> for u8 {
    #[inline(always)]
    fn from(variant: NegotiatedLinkWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NegotiatedLinkWidth {
    type Ux = u8;
}
#[doc = "Field `NEGOTIATED_LINK_WIDTH` reader - Negotiated link width Current link width are as follows:"]
pub type NegotiatedLinkWidthR = crate::FieldReader<NegotiatedLinkWidth>;
impl NegotiatedLinkWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NegotiatedLinkWidth> {
        match self.bits {
            2 => Some(NegotiatedLinkWidth::B10),
            1 => Some(NegotiatedLinkWidth::B01),
            0 => Some(NegotiatedLinkWidth::B00),
            _ => None,
        }
    }
    #[doc = "x1 others: Reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == NegotiatedLinkWidth::B10
    }
    #[doc = "x1 others: Reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == NegotiatedLinkWidth::B01
    }
    #[doc = "x1 others: Reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == NegotiatedLinkWidth::B00
    }
}
#[doc = "Field `MAX_RDREQ_SIZE` reader - Max read request size The maximum request size field programmed in the PCI Express Device Control Register. In multi?Function cores, this output provides the minimum of the max-read-request field in the Device Control Registers of all the Physical Functions. The client logic must limit the size of outgoing read requests to this value. The 3-bit codes are the same as those defined in PCIe Specifications: 3'b000 = 128 bytes 3'b001 = 256 bytes 3'b010 = 512 bytes 3'b011 = 1024 bytes 3'b100 = 2048 bytes 3'b101 = 4096 bytes"]
pub type MaxRdreqSizeR = crate::FieldReader;
#[doc = "Field `MAX_PAYLOAD_SIZE` reader - Max payload size The maximum payload size field programmed in the PCI Express Device Control Register. In multiple function cores, this output provides the minimum of the max-payload-size field in the Device Control Registers of all the enabled Physical Functions. The client logic must limit the size of Outgoing Completion payloads to this value. The 3-bit codes are the same as those defined in PCIe Specifications: 3'b000 = 128 bytes 3'b001 = 256 bytes 3'b010 = 512 bytes 3'b011 = 1024 bytes 3'b100 = 2048 bytes 3'b101 = 4096 bytes"]
pub type MaxPayloadSizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Optimized buffer flush and fill enable This output reflects the setting of the OBFF Enable field in the Device Control 2 Register"]
    #[inline(always)]
    pub fn obff_en(&self) -> ObffEnR {
        ObffEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Latency tolerance reporting mechanism enable The state of this output reflects the setting of the LTR Mechanism Enable bit in the Device Control 2 Register of Physical Function 0. When the core is configured as an Endpoint, client logic uses this output to enable the generation of LTR messages. This output is not to be used when the core is configured as a Root Complex."]
    #[inline(always)]
    pub fn ltr_en(&self) -> LtrEnR {
        LtrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read completion boundary status Provides the setting of the Read Completion Boundary (RCB) bit in the Link Control Register of each Physical Function. In the Endpoint mode, bit 0 indicates the RCB for PF 0 and so on. In the RC mode, bit 0 indicates the RCB setting of the Link Control Register of the RC. For each bit, a value of"]
    #[inline(always)]
    pub fn rcb_st(&self) -> RcbStR {
        RcbStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Operation speed after negotiation Current operating speed of the link is as follows:"]
    #[inline(always)]
    pub fn negotiated_speed(&self) -> NegotiatedSpeedR {
        NegotiatedSpeedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Negotiated link width Current link width are as follows:"]
    #[inline(always)]
    pub fn negotiated_link_width(&self) -> NegotiatedLinkWidthR {
        NegotiatedLinkWidthR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Max read request size The maximum request size field programmed in the PCI Express Device Control Register. In multi?Function cores, this output provides the minimum of the max-read-request field in the Device Control Registers of all the Physical Functions. The client logic must limit the size of outgoing read requests to this value. The 3-bit codes are the same as those defined in PCIe Specifications: 3'b000 = 128 bytes 3'b001 = 256 bytes 3'b010 = 512 bytes 3'b011 = 1024 bytes 3'b100 = 2048 bytes 3'b101 = 4096 bytes"]
    #[inline(always)]
    pub fn max_rdreq_size(&self) -> MaxRdreqSizeR {
        MaxRdreqSizeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Max payload size The maximum payload size field programmed in the PCI Express Device Control Register. In multiple function cores, this output provides the minimum of the max-payload-size field in the Device Control Registers of all the enabled Physical Functions. The client logic must limit the size of Outgoing Completion payloads to this value. The 3-bit codes are the same as those defined in PCIe Specifications: 3'b000 = 128 bytes 3'b001 = 256 bytes 3'b010 = 512 bytes 3'b011 = 1024 bytes 3'b100 = 2048 bytes 3'b101 = 4096 bytes"]
    #[inline(always)]
    pub fn max_payload_size(&self) -> MaxPayloadSizeR {
        MaxPayloadSizeR::new(((self.bits >> 12) & 7) as u8)
    }
}
#[doc = "Basic status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_basic_status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientBasicStatus0Spec;
impl crate::RegisterSpec for PcieClientBasicStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_basic_status0::R`](R) reader structure"]
impl crate::Readable for PcieClientBasicStatus0Spec {}
#[doc = "`reset()` method sets PCIE_CLIENT_BASIC_STATUS0 to value 0x0280"]
impl crate::Resettable for PcieClientBasicStatus0Spec {
    const RESET_VALUE: u32 = 0x0280;
}
