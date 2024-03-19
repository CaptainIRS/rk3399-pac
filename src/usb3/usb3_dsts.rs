#[doc = "Register `USB3_DSTS` reader"]
pub type R = crate::R<Usb3DstsSpec>;
#[doc = "Connected Speed\n\nIndicates the speed at which the DWC_usb3 core has come up\n\nafter speed detection through a chirp sequence.\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Connectspd {
    #[doc = "4: SuperSpeed (PHY clock is running at 125 or 250 MHz)"]
    B100 = 4,
    #[doc = "0: High-speed (PHY clock is running at 30 or 60 MHz)"]
    B000 = 0,
    #[doc = "1: Full-speed (PHY clock is running at 30 or 60 MHz)"]
    B001 = 1,
}
impl From<Connectspd> for u8 {
    #[inline(always)]
    fn from(variant: Connectspd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Connectspd {
    type Ux = u8;
}
#[doc = "Field `CONNECTSPD` reader - Connected Speed\n\nIndicates the speed at which the DWC_usb3 core has come up\n\nafter speed detection through a chirp sequence."]
pub type ConnectspdR = crate::FieldReader<Connectspd>;
impl ConnectspdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Connectspd> {
        match self.bits {
            4 => Some(Connectspd::B100),
            0 => Some(Connectspd::B000),
            1 => Some(Connectspd::B001),
            _ => None,
        }
    }
    #[doc = "SuperSpeed (PHY clock is running at 125 or 250 MHz)"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Connectspd::B100
    }
    #[doc = "High-speed (PHY clock is running at 30 or 60 MHz)"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Connectspd::B000
    }
    #[doc = "Full-speed (PHY clock is running at 30 or 60 MHz)"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Connectspd::B001
    }
}
#[doc = "Field `USBLNKST` reader - USBLNKST\n\nSS mode: LTSSM State\n\n4'h0: U0\n\n4'h1: U1\n\n4'h2: U2\n\n4'h3: U3\n\n4'h5: RX_DET\n\n4'h6: SS_INACT\n\n4'h7: POLL\n\n4'h8: RECOV\n\n4'h9: HRESET\n\n4'ha: CMPLY\n\n4'hb: LPBK\n\n4'hf: Resume/Reset\n\nIn HS/FS/LS mode:\n\n4'h0: On state\n\n4'h2: Sleep (L1) state\n\n4'h3: Suspend (L2) state\n\n4'h4: Disconnected state (Default state)\n\n4'h5: Early Suspend state (valid only when Hibernation is\n\ndisabled, GCTL\\[1\\].GblHibernationEn = 0)\n\n4'he: Reset (valid only when Hibernation is enabled,\n\nGCTL\\[1\\].GblHibernationEn = 1)\n\n4'hf: Resume (valid only when Hibernation is enabled,\n\nGCTL\\[1\\].GblHibernationEn = 1)\n\nThe link state Resume/Reset indicates that the core received a\n\nresume or USB reset request from the host while the link was in\n\nhibernation. Software must write 8 (Recovery) to the\n\nDCTL.ULStChngReq field to acknowledge the resume/reset\n\nrequest.\n\nWhen Hibernation is enabled, GCTL\\[1\\].GblHibernationEn = 1, this\n\nfield USBLnkSt is valid only when DCTL\\[31\\].Run/Stop set to 1\n\nand DSTS\\[29\\].DCNRD = 0."]
pub type UsblnkstR = crate::FieldReader;
#[doc = "Field `DEVCTRLHLT` reader - Device Controller Halted\n\nThis bit is set to 0 when the Run/Stop bit in the DCTL register is\n\nset to 1.\n\nThe core sets this bit to 1 when, after SW sets Run/Stop to 0, the\n\ncore is idle and the lower layer finishes the disconnect process.\n\nWhen Halted=1, the core does not generate Device events.\n\nNote: The core does not set this bit to 1 if GEVNTCOUNTn has\n\nsome valid value. Software needs to acknowledge the events that\n\nare generated (by writing to GEVNTCOUNTn) while it is waiting\n\nfor this bit to be set to 1."]
pub type DevctrlhltR = crate::BitReader;
#[doc = "Field `COREIDLE` reader - Core Idle\n\nThe bit indicates that the core finished transferring all RxFIFO\n\ndata to system memory, writing out all completed descriptors,\n\nand all Event Counts are zero.\n\nNote: While testing for Reset values, mask out the read value.\n\nThis bit represents the changing state of the core and does not\n\nhold a static value."]
pub type CoreidleR = crate::BitReader;
#[doc = "Field `SSS` reader - SSS Save State Status\n\nThis bit is similar to the USBSTS.SSS in host mode.\n\nWhen the controller has finished the save process, it completes\n\nthe command by setting DSTS.SSS to 0."]
pub type SssR = crate::BitReader;
#[doc = "Field `RSS` reader - RSS Restore State Status\n\nThis bit is similar to the USBSTS.RSS in host mode.\n\nWhen the controller finishes the restore process, it completes the\n\ncommand by setting DSTS.RSS to 0."]
pub type RssR = crate::BitReader;
#[doc = "Field `DCNRD` reader - Device Controller Not Ready\n\nThe bit indicates that the core is in the process of completing the\n\nstate transitions after exiting from hibernation.\n\nTo complete the state transitions, it takes 256 bus clock cycles\n\nfrom the time DCTL\\[31\\].Run/Stop is set. During hibernation, if\n\nthe UTMI/ULPI PHY is in suspended state, then the 256-bus clock\n\ncycle delay starts after the PHY exited suspended state. Software\n\nmust set DCTL\\[31\\].Run/Stop to 1 and wait for this bit to be de-\n\nasserted to zero before processing DSTS.USBLnkSt."]
pub type DcnrdR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Connected Speed\n\nIndicates the speed at which the DWC_usb3 core has come up\n\nafter speed detection through a chirp sequence."]
    #[inline(always)]
    pub fn connectspd(&self) -> ConnectspdR {
        ConnectspdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 18:21 - USBLNKST\n\nSS mode: LTSSM State\n\n4'h0: U0\n\n4'h1: U1\n\n4'h2: U2\n\n4'h3: U3\n\n4'h5: RX_DET\n\n4'h6: SS_INACT\n\n4'h7: POLL\n\n4'h8: RECOV\n\n4'h9: HRESET\n\n4'ha: CMPLY\n\n4'hb: LPBK\n\n4'hf: Resume/Reset\n\nIn HS/FS/LS mode:\n\n4'h0: On state\n\n4'h2: Sleep (L1) state\n\n4'h3: Suspend (L2) state\n\n4'h4: Disconnected state (Default state)\n\n4'h5: Early Suspend state (valid only when Hibernation is\n\ndisabled, GCTL\\[1\\].GblHibernationEn = 0)\n\n4'he: Reset (valid only when Hibernation is enabled,\n\nGCTL\\[1\\].GblHibernationEn = 1)\n\n4'hf: Resume (valid only when Hibernation is enabled,\n\nGCTL\\[1\\].GblHibernationEn = 1)\n\nThe link state Resume/Reset indicates that the core received a\n\nresume or USB reset request from the host while the link was in\n\nhibernation. Software must write 8 (Recovery) to the\n\nDCTL.ULStChngReq field to acknowledge the resume/reset\n\nrequest.\n\nWhen Hibernation is enabled, GCTL\\[1\\].GblHibernationEn = 1, this\n\nfield USBLnkSt is valid only when DCTL\\[31\\].Run/Stop set to 1\n\nand DSTS\\[29\\].DCNRD = 0."]
    #[inline(always)]
    pub fn usblnkst(&self) -> UsblnkstR {
        UsblnkstR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Device Controller Halted\n\nThis bit is set to 0 when the Run/Stop bit in the DCTL register is\n\nset to 1.\n\nThe core sets this bit to 1 when, after SW sets Run/Stop to 0, the\n\ncore is idle and the lower layer finishes the disconnect process.\n\nWhen Halted=1, the core does not generate Device events.\n\nNote: The core does not set this bit to 1 if GEVNTCOUNTn has\n\nsome valid value. Software needs to acknowledge the events that\n\nare generated (by writing to GEVNTCOUNTn) while it is waiting\n\nfor this bit to be set to 1."]
    #[inline(always)]
    pub fn devctrlhlt(&self) -> DevctrlhltR {
        DevctrlhltR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Core Idle\n\nThe bit indicates that the core finished transferring all RxFIFO\n\ndata to system memory, writing out all completed descriptors,\n\nand all Event Counts are zero.\n\nNote: While testing for Reset values, mask out the read value.\n\nThis bit represents the changing state of the core and does not\n\nhold a static value."]
    #[inline(always)]
    pub fn coreidle(&self) -> CoreidleR {
        CoreidleR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SSS Save State Status\n\nThis bit is similar to the USBSTS.SSS in host mode.\n\nWhen the controller has finished the save process, it completes\n\nthe command by setting DSTS.SSS to 0."]
    #[inline(always)]
    pub fn sss(&self) -> SssR {
        SssR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RSS Restore State Status\n\nThis bit is similar to the USBSTS.RSS in host mode.\n\nWhen the controller finishes the restore process, it completes the\n\ncommand by setting DSTS.RSS to 0."]
    #[inline(always)]
    pub fn rss(&self) -> RssR {
        RssR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Device Controller Not Ready\n\nThe bit indicates that the core is in the process of completing the\n\nstate transitions after exiting from hibernation.\n\nTo complete the state transitions, it takes 256 bus clock cycles\n\nfrom the time DCTL\\[31\\].Run/Stop is set. During hibernation, if\n\nthe UTMI/ULPI PHY is in suspended state, then the 256-bus clock\n\ncycle delay starts after the PHY exited suspended state. Software\n\nmust set DCTL\\[31\\].Run/Stop to 1 and wait for this bit to be de-\n\nasserted to zero before processing DSTS.USBLnkSt."]
    #[inline(always)]
    pub fn dcnrd(&self) -> DcnrdR {
        DcnrdR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Device Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3DstsSpec;
impl crate::RegisterSpec for Usb3DstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_dsts::R`](R) reader structure"]
impl crate::Readable for Usb3DstsSpec {}
#[doc = "`reset()` method sets USB3_DSTS to value 0x0050_0004"]
impl crate::Resettable for Usb3DstsSpec {
    const RESET_VALUE: u32 = 0x0050_0004;
}
