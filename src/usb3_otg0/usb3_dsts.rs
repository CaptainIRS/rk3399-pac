#[doc = "Register `USB3_DSTS` reader"]
pub type R = crate::R<Usb3DstsSpec>;
#[doc = "Field `CONNECTSPD` reader - Connected Speed Indicates the speed at which the DWC_usb3 core has come up after speed detection through a chirp sequence. 3'b100: SuperSpeed (PHY clock is running at 125 or 250 MHz) 3'b000: High-speed (PHY clock is running at 30 or 60 MHz) 3'b001: Full-speed (PHY clock is running at 30 or 60 MHz)"]
pub type ConnectspdR = crate::FieldReader;
#[doc = "Field `USBLNKST` reader - USBLNKST SS mode: LTSSM State 4'h0: U0 4'h1: U1 4'h2: U2 4'h3: U3 4'h5: RX_DET 4'h6: SS_INACT 4'h7: POLL 4'h8: RECOV 4'h9: HRESET 4'ha: CMPLY 4'hb: LPBK 4'hf: Resume/Reset In HS/FS/LS mode: 4'h0: On state 4'h2: Sleep (L1) state 4'h3: Suspend (L2) state 4'h4: Disconnected state (Default state) 4'h5: Early Suspend state (valid only when Hibernation is disabled, GCTL\\[1\\].GblHibernationEn = 0) 4'he: Reset (valid only when Hibernation is enabled, GCTL\\[1\\].GblHibernationEn = 1) 4'hf: Resume (valid only when Hibernation is enabled, GCTL\\[1\\].GblHibernationEn = 1) The link state Resume/Reset indicates that the core received a resume or USB reset request from the host while the link was in hibernation. Software must write 8 (Recovery) to the DCTL.ULStChngReq field to acknowledge the resume/reset request. When Hibernation is enabled, GCTL\\[1\\].GblHibernationEn = 1, this field USBLnkSt is valid only when DCTL\\[31\\].Run/Stop set to 1 and DSTS\\[29\\].DCNRD = 0."]
pub type UsblnkstR = crate::FieldReader;
#[doc = "Field `DEVCTRLHLT` reader - Device Controller Halted This bit is set to 0 when the Run/Stop bit in the DCTL register is set to 1. The core sets this bit to 1 when, after SW sets Run/Stop to 0, the core is idle and the lower layer finishes the disconnect process. When Halted=1, the core does not generate Device events. Note: The core does not set this bit to 1 if GEVNTCOUNTn has some valid value. Software needs to acknowledge the events that are generated (by writing to GEVNTCOUNTn) while it is waiting for this bit to be set to 1."]
pub type DevctrlhltR = crate::BitReader;
#[doc = "Field `COREIDLE` reader - Core Idle The bit indicates that the core finished transferring all RxFIFO data to system memory, writing out all completed descriptors, and all Event Counts are zero. Note: While testing for Reset values, mask out the read value. This bit represents the changing state of the core and does not hold a static value."]
pub type CoreidleR = crate::BitReader;
#[doc = "Field `SSS` reader - SSS Save State Status This bit is similar to the USBSTS.SSS in host mode. When the controller has finished the save process, it completes the command by setting DSTS.SSS to 0."]
pub type SssR = crate::BitReader;
#[doc = "Field `RSS` reader - RSS Restore State Status This bit is similar to the USBSTS.RSS in host mode. When the controller finishes the restore process, it completes the command by setting DSTS.RSS to 0."]
pub type RssR = crate::BitReader;
#[doc = "Field `DCNRD` reader - Device Controller Not Ready The bit indicates that the core is in the process of completing the state transitions after exiting from hibernation. To complete the state transitions, it takes 256 bus clock cycles from the time DCTL\\[31\\].Run/Stop is set. During hibernation, if the UTMI/ULPI PHY is in suspended state, then the 256-bus clock cycle delay starts after the PHY exited suspended state. Software must set DCTL\\[31\\].Run/Stop to 1 and wait for this bit to be de- asserted to zero before processing DSTS.USBLnkSt."]
pub type DcnrdR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Connected Speed Indicates the speed at which the DWC_usb3 core has come up after speed detection through a chirp sequence. 3'b100: SuperSpeed (PHY clock is running at 125 or 250 MHz) 3'b000: High-speed (PHY clock is running at 30 or 60 MHz) 3'b001: Full-speed (PHY clock is running at 30 or 60 MHz)"]
    #[inline(always)]
    pub fn connectspd(&self) -> ConnectspdR {
        ConnectspdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 18:21 - USBLNKST SS mode: LTSSM State 4'h0: U0 4'h1: U1 4'h2: U2 4'h3: U3 4'h5: RX_DET 4'h6: SS_INACT 4'h7: POLL 4'h8: RECOV 4'h9: HRESET 4'ha: CMPLY 4'hb: LPBK 4'hf: Resume/Reset In HS/FS/LS mode: 4'h0: On state 4'h2: Sleep (L1) state 4'h3: Suspend (L2) state 4'h4: Disconnected state (Default state) 4'h5: Early Suspend state (valid only when Hibernation is disabled, GCTL\\[1\\].GblHibernationEn = 0) 4'he: Reset (valid only when Hibernation is enabled, GCTL\\[1\\].GblHibernationEn = 1) 4'hf: Resume (valid only when Hibernation is enabled, GCTL\\[1\\].GblHibernationEn = 1) The link state Resume/Reset indicates that the core received a resume or USB reset request from the host while the link was in hibernation. Software must write 8 (Recovery) to the DCTL.ULStChngReq field to acknowledge the resume/reset request. When Hibernation is enabled, GCTL\\[1\\].GblHibernationEn = 1, this field USBLnkSt is valid only when DCTL\\[31\\].Run/Stop set to 1 and DSTS\\[29\\].DCNRD = 0."]
    #[inline(always)]
    pub fn usblnkst(&self) -> UsblnkstR {
        UsblnkstR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Device Controller Halted This bit is set to 0 when the Run/Stop bit in the DCTL register is set to 1. The core sets this bit to 1 when, after SW sets Run/Stop to 0, the core is idle and the lower layer finishes the disconnect process. When Halted=1, the core does not generate Device events. Note: The core does not set this bit to 1 if GEVNTCOUNTn has some valid value. Software needs to acknowledge the events that are generated (by writing to GEVNTCOUNTn) while it is waiting for this bit to be set to 1."]
    #[inline(always)]
    pub fn devctrlhlt(&self) -> DevctrlhltR {
        DevctrlhltR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Core Idle The bit indicates that the core finished transferring all RxFIFO data to system memory, writing out all completed descriptors, and all Event Counts are zero. Note: While testing for Reset values, mask out the read value. This bit represents the changing state of the core and does not hold a static value."]
    #[inline(always)]
    pub fn coreidle(&self) -> CoreidleR {
        CoreidleR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SSS Save State Status This bit is similar to the USBSTS.SSS in host mode. When the controller has finished the save process, it completes the command by setting DSTS.SSS to 0."]
    #[inline(always)]
    pub fn sss(&self) -> SssR {
        SssR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RSS Restore State Status This bit is similar to the USBSTS.RSS in host mode. When the controller finishes the restore process, it completes the command by setting DSTS.RSS to 0."]
    #[inline(always)]
    pub fn rss(&self) -> RssR {
        RssR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Device Controller Not Ready The bit indicates that the core is in the process of completing the state transitions after exiting from hibernation. To complete the state transitions, it takes 256 bus clock cycles from the time DCTL\\[31\\].Run/Stop is set. During hibernation, if the UTMI/ULPI PHY is in suspended state, then the 256-bus clock cycle delay starts after the PHY exited suspended state. Software must set DCTL\\[31\\].Run/Stop to 1 and wait for this bit to be de- asserted to zero before processing DSTS.USBLnkSt."]
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
