#[doc = "Register `GUCTL1` reader"]
pub type R = crate::R<Guctl1Spec>;
#[doc = "Register `GUCTL1` writer"]
pub type W = crate::W<Guctl1Spec>;
#[doc = "Field `LOA_FILTER_EN` reader - LOA_FILTER_EN\n\nIf this bit is set, the USB 2.0 port babble is checked at least three\n\nconsecutive times before the port is disabled. This prevents false\n\ntriggering of the babble condition when using low quality cables.\n\nNote: This bit is valid only in host mode."]
pub type LoaFilterEnR = crate::BitReader;
#[doc = "Field `LOA_FILTER_EN` writer - LOA_FILTER_EN\n\nIf this bit is set, the USB 2.0 port babble is checked at least three\n\nconsecutive times before the port is disabled. This prevents false\n\ntriggering of the babble condition when using low quality cables.\n\nNote: This bit is valid only in host mode."]
pub type LoaFilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRLD_L1_SUSP_COM` reader - OVRLD_L1_SUSP_COM\n\nIf this bit is set, the utmi_l1_suspend_com_n is overloaded with\n\nthe utmi_sleep_n signal. This bit is usually set if the PHY stops\n\nthe port clock during L1 sleep condition."]
pub type OvrldL1SuspComR = crate::BitReader;
#[doc = "Field `OVRLD_L1_SUSP_COM` writer - OVRLD_L1_SUSP_COM\n\nIf this bit is set, the utmi_l1_suspend_com_n is overloaded with\n\nthe utmi_sleep_n signal. This bit is usually set if the PHY stops\n\nthe port clock during L1 sleep condition."]
pub type OvrldL1SuspComW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HC_PARCHK_DISABLE` reader - Host Parameter Check Disable\n\nWhen this bit is set to 0 (by default), the xHC checks that the\n\ninput slot/EP context fields comply to the xHCI Specification.\n\nUpon detection of a parameter error during command execution,\n\nthe xHC generates an event TRB with completion code indicating\n\nPARAMETER ERROR.\n\nWhen the bit is set to 1, the xHC does not perform parameter\n\nchecks and does not generate PARAMETER ERROR completion\n\ncode."]
pub type HcParchkDisableR = crate::BitReader;
#[doc = "Field `HC_PARCHK_DISABLE` writer - Host Parameter Check Disable\n\nWhen this bit is set to 0 (by default), the xHC checks that the\n\ninput slot/EP context fields comply to the xHCI Specification.\n\nUpon detection of a parameter error during command execution,\n\nthe xHC generates an event TRB with completion code indicating\n\nPARAMETER ERROR.\n\nWhen the bit is set to 1, the xHC does not perform parameter\n\nchecks and does not generate PARAMETER ERROR completion\n\ncode."]
pub type HcParchkDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HC_ERRATA_ENABLE` reader - Host ELD Enable\n\nWhen this bit is set to 1, it enables the Exit Latency Delta (ELD)\n\nsupport defined in the xHCI 1.0 Errata.\n\nThis bit is used only in the host mode. This bit has to be set to 1\n\nin Host mode."]
pub type HcErrataEnableR = crate::BitReader;
#[doc = "Field `HC_ERRATA_ENABLE` writer - Host ELD Enable\n\nWhen this bit is set to 1, it enables the Exit Latency Delta (ELD)\n\nsupport defined in the xHCI 1.0 Errata.\n\nThis bit is used only in the host mode. This bit has to be set to 1\n\nin Host mode."]
pub type HcErrataEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_SUSP_THRLD_FOR_HOST` reader - L1_SUSP_THRLD_FOR_HOST\n\nThis field is effective only when the\n\nL1_SUSP_THRLD_EN_FOR_HOST bit is set to 1. For more details,\n\nrefer to the description of the L1_SUSP_THRLD_EN_FOR_HOST\n\nbit."]
pub type L1SuspThrldForHostR = crate::FieldReader;
#[doc = "Field `L1_SUSP_THRLD_FOR_HOST` writer - L1_SUSP_THRLD_FOR_HOST\n\nThis field is effective only when the\n\nL1_SUSP_THRLD_EN_FOR_HOST bit is set to 1. For more details,\n\nrefer to the description of the L1_SUSP_THRLD_EN_FOR_HOST\n\nbit."]
pub type L1SuspThrldForHostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `L1_SUSP_THRLD_EN_FOR_HOST` reader - L1_SUSP_THRLD_EN_FOR_HOST\n\nThis bit is used only in host mode.\n\nThe host controller asserts the utmi_l1_suspend_n and\n\nutmi_sleep_n output signals (see LPM Interface Signals table in\n\nthe Databook) as follows:\n\nThe controller asserts the utmi_l1_suspend_n signal to put the\n\nPHY into deep low-power mode in L1 when both of the following\n\nare true:\n\nThe HIRD/BESL value used is greater than or equal to the value\n\nin L1_SUSP_THRLD_FOR_HOST field.\n\nThe L1_SUSP_THRLD_EN_FOR_HOST bit is set to 1'b1. The\n\ncontroller asserts utmi_sleep_n on L1 when one of the following\n\nis true:\n\nThe HIRD/BESL value used is less than the value in\n\nL1_SUSP_THRLD_FOR_HOST field.\n\nThe L1_SUSP_THRLD_EN_FOR_HOST bit is set to 1'b0."]
pub type L1SuspThrldEnForHostR = crate::BitReader;
#[doc = "Field `L1_SUSP_THRLD_EN_FOR_HOST` writer - L1_SUSP_THRLD_EN_FOR_HOST\n\nThis bit is used only in host mode.\n\nThe host controller asserts the utmi_l1_suspend_n and\n\nutmi_sleep_n output signals (see LPM Interface Signals table in\n\nthe Databook) as follows:\n\nThe controller asserts the utmi_l1_suspend_n signal to put the\n\nPHY into deep low-power mode in L1 when both of the following\n\nare true:\n\nThe HIRD/BESL value used is greater than or equal to the value\n\nin L1_SUSP_THRLD_FOR_HOST field.\n\nThe L1_SUSP_THRLD_EN_FOR_HOST bit is set to 1'b1. The\n\ncontroller asserts utmi_sleep_n on L1 when one of the following\n\nis true:\n\nThe HIRD/BESL value used is less than the value in\n\nL1_SUSP_THRLD_FOR_HOST field.\n\nThe L1_SUSP_THRLD_EN_FOR_HOST bit is set to 1'b0."]
pub type L1SuspThrldEnForHostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARKMODE_DISABLE_FSLS` reader - PARKMODE_DISABLE_FSLS\n\nThis bit is used only in host mode, and is for debug purpose only.\n\nWhen this bit is set to 1 all FS/LS bus instances in park mode\n\ndisabled."]
pub type ParkmodeDisableFslsR = crate::BitReader;
#[doc = "Field `PARKMODE_DISABLE_FSLS` writer - PARKMODE_DISABLE_FSLS\n\nThis bit is used only in host mode, and is for debug purpose only.\n\nWhen this bit is set to 1 all FS/LS bus instances in park mode\n\ndisabled."]
pub type ParkmodeDisableFslsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARKMODE_DISABLE_HS` reader - PARKMODE_DISABLE_HS\n\nThis bit is used only in host mode.\n\nWhen this bit is set to 1 all HS bus instances park mode are\n\ndisabled.\n\nTo improve performance in park mode, the xHCI scheduler\n\nqueues in three requests of 4 packets each for High Speed\n\nasynchronous endpoints in a micro-frame. But if a device is slow\n\nand if it NAKs more than 3 times, then it is rescheduled only in\n\nthe next micro-frame. This could decrease the performance of a\n\nslow device even further.\n\nIn a few high speed devices (such as Sandisk Cruzer Blade 4GB\n\nVID:1921, PID:21863 and Flex Drive VID:3744, PID:8552) when\n\nan IN request is sent within 900ns of the ACK of the previous\n\npacket, these devices send a NAK. When connected to these\n\ndevices, if required, the software can disable the park mode if\n\nyou see performance drop in your system. When park mode is\n\ndisabled, pipelining of multiple packet is disabled and instead one\n\npacket at a time is requested by the scheduler. This allows up to\n\n12 NAKs in a micro-frame and improves performance of these\n\nslow devices."]
pub type ParkmodeDisableHsR = crate::BitReader;
#[doc = "Field `PARKMODE_DISABLE_HS` writer - PARKMODE_DISABLE_HS\n\nThis bit is used only in host mode.\n\nWhen this bit is set to 1 all HS bus instances park mode are\n\ndisabled.\n\nTo improve performance in park mode, the xHCI scheduler\n\nqueues in three requests of 4 packets each for High Speed\n\nasynchronous endpoints in a micro-frame. But if a device is slow\n\nand if it NAKs more than 3 times, then it is rescheduled only in\n\nthe next micro-frame. This could decrease the performance of a\n\nslow device even further.\n\nIn a few high speed devices (such as Sandisk Cruzer Blade 4GB\n\nVID:1921, PID:21863 and Flex Drive VID:3744, PID:8552) when\n\nan IN request is sent within 900ns of the ACK of the previous\n\npacket, these devices send a NAK. When connected to these\n\ndevices, if required, the software can disable the park mode if\n\nyou see performance drop in your system. When park mode is\n\ndisabled, pipelining of multiple packet is disabled and instead one\n\npacket at a time is requested by the scheduler. This allows up to\n\n12 NAKs in a micro-frame and improves performance of these\n\nslow devices."]
pub type ParkmodeDisableHsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARKMODE_DISABLE_SS` reader - PARKMODE_DISABLE_SS\n\nThis bit is used only in host mode, and is for debug purpose only.\n\nWhen this bit is set to 1 all SS bus instances in park mode are\n\ndisabled."]
pub type ParkmodeDisableSsR = crate::BitReader;
#[doc = "Field `PARKMODE_DISABLE_SS` writer - PARKMODE_DISABLE_SS\n\nThis bit is used only in host mode, and is for debug purpose only.\n\nWhen this bit is set to 1 all SS bus instances in park mode are\n\ndisabled."]
pub type ParkmodeDisableSsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "NAK_PER_ENH_HS\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NakPerEnhHs {
    #[doc = "1: Enables performance enhancement for HS async endpoints in the presence of NAKs"]
    B1 = 1,
    #[doc = "0: Enhancement not applied If a periodic endpoint is present , and if a bulk endpoint which is also active is being NAKed by the device, then this could result in a decrease in performance of other High Speed bulk endpoint which is ACked by the device. Setting this bit to 1, will enable the host controller to schedule more transactions to the async endpoints (bulk/ control) and hence will improve the performance of the bulk endpoint. This control bit should be enabled only if the existing performance with the default setting is not sufficient for your HighSpeed application. Setting this bit will only control, and is only required for High Speed transfers."]
    B0 = 0,
}
impl From<NakPerEnhHs> for bool {
    #[inline(always)]
    fn from(variant: NakPerEnhHs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAK_PER_ENH_HS` reader - NAK_PER_ENH_HS"]
pub type NakPerEnhHsR = crate::BitReader<NakPerEnhHs>;
impl NakPerEnhHsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NakPerEnhHs {
        match self.bits {
            true => NakPerEnhHs::B1,
            false => NakPerEnhHs::B0,
        }
    }
    #[doc = "Enables performance enhancement for HS async endpoints in the presence of NAKs"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NakPerEnhHs::B1
    }
    #[doc = "Enhancement not applied If a periodic endpoint is present , and if a bulk endpoint which is also active is being NAKed by the device, then this could result in a decrease in performance of other High Speed bulk endpoint which is ACked by the device. Setting this bit to 1, will enable the host controller to schedule more transactions to the async endpoints (bulk/ control) and hence will improve the performance of the bulk endpoint. This control bit should be enabled only if the existing performance with the default setting is not sufficient for your HighSpeed application. Setting this bit will only control, and is only required for High Speed transfers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NakPerEnhHs::B0
    }
}
#[doc = "Field `NAK_PER_ENH_HS` writer - NAK_PER_ENH_HS"]
pub type NakPerEnhHsW<'a, REG> = crate::BitWriter<'a, REG, NakPerEnhHs>;
impl<'a, REG> NakPerEnhHsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enables performance enhancement for HS async endpoints in the presence of NAKs"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(NakPerEnhHs::B1)
    }
    #[doc = "Enhancement not applied If a periodic endpoint is present , and if a bulk endpoint which is also active is being NAKed by the device, then this could result in a decrease in performance of other High Speed bulk endpoint which is ACked by the device. Setting this bit to 1, will enable the host controller to schedule more transactions to the async endpoints (bulk/ control) and hence will improve the performance of the bulk endpoint. This control bit should be enabled only if the existing performance with the default setting is not sufficient for your HighSpeed application. Setting this bit will only control, and is only required for High Speed transfers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(NakPerEnhHs::B0)
    }
}
#[doc = "NAK_PER_ENH_FS\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NakPerEnhFs {
    #[doc = "1: Enables performance enhancement for FS async endpoints in the presence of NAKs"]
    B1 = 1,
    #[doc = "0: Enhancement not applied If a periodic endpoint is present , and if a bulk endpoint which is also active is being NAKed by the device, then this could result in a decrease in performance of other Full Speed bulk endpoint which is ACked by the device. Setting this bit to 1, will enable the host controller to schedule more transactions to the async endpoints (bulk/ control) and hence will improve the performance of the bulk endpoint. This control bit should be enabled only if the existing performance with the default setting is not sufficient for your FullSpeed application. Setting this bit will only control, and is only required for Full Speed transfers."]
    B0 = 0,
}
impl From<NakPerEnhFs> for bool {
    #[inline(always)]
    fn from(variant: NakPerEnhFs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAK_PER_ENH_FS` reader - NAK_PER_ENH_FS"]
pub type NakPerEnhFsR = crate::BitReader<NakPerEnhFs>;
impl NakPerEnhFsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NakPerEnhFs {
        match self.bits {
            true => NakPerEnhFs::B1,
            false => NakPerEnhFs::B0,
        }
    }
    #[doc = "Enables performance enhancement for FS async endpoints in the presence of NAKs"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NakPerEnhFs::B1
    }
    #[doc = "Enhancement not applied If a periodic endpoint is present , and if a bulk endpoint which is also active is being NAKed by the device, then this could result in a decrease in performance of other Full Speed bulk endpoint which is ACked by the device. Setting this bit to 1, will enable the host controller to schedule more transactions to the async endpoints (bulk/ control) and hence will improve the performance of the bulk endpoint. This control bit should be enabled only if the existing performance with the default setting is not sufficient for your FullSpeed application. Setting this bit will only control, and is only required for Full Speed transfers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NakPerEnhFs::B0
    }
}
#[doc = "Field `NAK_PER_ENH_FS` writer - NAK_PER_ENH_FS"]
pub type NakPerEnhFsW<'a, REG> = crate::BitWriter<'a, REG, NakPerEnhFs>;
impl<'a, REG> NakPerEnhFsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enables performance enhancement for FS async endpoints in the presence of NAKs"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(NakPerEnhFs::B1)
    }
    #[doc = "Enhancement not applied If a periodic endpoint is present , and if a bulk endpoint which is also active is being NAKed by the device, then this could result in a decrease in performance of other Full Speed bulk endpoint which is ACked by the device. Setting this bit to 1, will enable the host controller to schedule more transactions to the async endpoints (bulk/ control) and hence will improve the performance of the bulk endpoint. This control bit should be enabled only if the existing performance with the default setting is not sufficient for your FullSpeed application. Setting this bit will only control, and is only required for Full Speed transfers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(NakPerEnhFs::B0)
    }
}
#[doc = "DEV_LSP_TAIL_LOCK_DIS\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevLspTailLockDis {
    #[doc = "0: Default behaviour, enables device lsp lock logic for tail TRB update"]
    B0 = 0,
    #[doc = "1: Fix disabled This is a bug fix for STAR 9000716195 that affects the CSP mode for OUT endpoints in device mode. The issue is that tail TRB index is not synchronized with the cache Scratchpad bytecount update. If the fast-forward request comes in-between the bytecount update ona newly fetched TRB and the tail-index write update in TPF, the RDP works on an incorrect tail index and misses the byte count decrement for the newly fetched TRB in the fast-forwarding process. This fix needs to be present all the times."]
    B1 = 1,
}
impl From<DevLspTailLockDis> for bool {
    #[inline(always)]
    fn from(variant: DevLspTailLockDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_LSP_TAIL_LOCK_DIS` reader - DEV_LSP_TAIL_LOCK_DIS"]
pub type DevLspTailLockDisR = crate::BitReader<DevLspTailLockDis>;
impl DevLspTailLockDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevLspTailLockDis {
        match self.bits {
            false => DevLspTailLockDis::B0,
            true => DevLspTailLockDis::B1,
        }
    }
    #[doc = "Default behaviour, enables device lsp lock logic for tail TRB update"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DevLspTailLockDis::B0
    }
    #[doc = "Fix disabled This is a bug fix for STAR 9000716195 that affects the CSP mode for OUT endpoints in device mode. The issue is that tail TRB index is not synchronized with the cache Scratchpad bytecount update. If the fast-forward request comes in-between the bytecount update ona newly fetched TRB and the tail-index write update in TPF, the RDP works on an incorrect tail index and misses the byte count decrement for the newly fetched TRB in the fast-forwarding process. This fix needs to be present all the times."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DevLspTailLockDis::B1
    }
}
#[doc = "Field `DEV_LSP_TAIL_LOCK_DIS` writer - DEV_LSP_TAIL_LOCK_DIS"]
pub type DevLspTailLockDisW<'a, REG> = crate::BitWriter<'a, REG, DevLspTailLockDis>;
impl<'a, REG> DevLspTailLockDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default behaviour, enables device lsp lock logic for tail TRB update"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DevLspTailLockDis::B0)
    }
    #[doc = "Fix disabled This is a bug fix for STAR 9000716195 that affects the CSP mode for OUT endpoints in device mode. The issue is that tail TRB index is not synchronized with the cache Scratchpad bytecount update. If the fast-forward request comes in-between the bytecount update ona newly fetched TRB and the tail-index write update in TPF, the RDP works on an incorrect tail index and misses the byte count decrement for the newly fetched TRB in the fast-forwarding process. This fix needs to be present all the times."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DevLspTailLockDis::B1)
    }
}
#[doc = "Field `IP_GAP_ADD_ON` reader - IP_GAP_ADD_ON\n\nThis register field is used to add on to the default inter packet\n\ngap setting in the USB 2.0 MAC. This should be programmed to a\n\nnon zero value only in case where you need to increase the\n\ndefault inter packet delay calculations in the USB 2.0 MAC\n\nmodule DWC_usb3_u2mac.v"]
pub type IpGapAddOnR = crate::FieldReader;
#[doc = "Field `IP_GAP_ADD_ON` writer - IP_GAP_ADD_ON\n\nThis register field is used to add on to the default inter packet\n\ngap setting in the USB 2.0 MAC. This should be programmed to a\n\nnon zero value only in case where you need to increase the\n\ndefault inter packet delay calculations in the USB 2.0 MAC\n\nmodule DWC_usb3_u2mac.v"]
pub type IpGapAddOnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "DEV_L1_EXIT_BY_HW\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevL1ExitByHw {
    #[doc = "0: Default behaviour, disables device L1 hardware exit logic"]
    B0 = 0,
    #[doc = "1: feature enabled This bit is applicable for device mode (2.0) only. This field enables device controller sending remote wakeup for L1 if the device becomes ready for sending/accepting data when in L1 state. If the host expects the device to send remote wkp signalling to resume after going into L1 in flow controlled state, then this bit can be set to send the remote wake signal automatically when the device controller becomes ready. This HW remote wake feature is applicable only to bulk and interrupt transfers, and not for Isoch/Control When control transfers are in progress, the LPM will be rejected (NYET response). Only after control transfers are completed (either with ACK/STALL), LPM will be accepted For Isoch transfers, the host needs to do the wake-up and start the transfer. Device controller will not do remote-wakeup when Isoch endpoints get ready. The device SW needs to keep the GUSB2PHYCFG\\[EnblSlpM\\]
reset in order to keep the PHY clock to be running for keeping track of SOF intervals. When L1 hibernation is enabled, the controller will not do automatic exit for hibernation requests thru L1. This bit is quasi-static, i.e., should not be changed during device operation."]
    B1 = 1,
}
impl From<DevL1ExitByHw> for bool {
    #[inline(always)]
    fn from(variant: DevL1ExitByHw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_L1_EXIT_BY_HW` reader - DEV_L1_EXIT_BY_HW"]
pub type DevL1ExitByHwR = crate::BitReader<DevL1ExitByHw>;
impl DevL1ExitByHwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevL1ExitByHw {
        match self.bits {
            false => DevL1ExitByHw::B0,
            true => DevL1ExitByHw::B1,
        }
    }
    #[doc = "Default behaviour, disables device L1 hardware exit logic"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DevL1ExitByHw::B0
    }
    #[doc = "feature enabled This bit is applicable for device mode (2.0) only. This field enables device controller sending remote wakeup for L1 if the device becomes ready for sending/accepting data when in L1 state. If the host expects the device to send remote wkp signalling to resume after going into L1 in flow controlled state, then this bit can be set to send the remote wake signal automatically when the device controller becomes ready. This HW remote wake feature is applicable only to bulk and interrupt transfers, and not for Isoch/Control When control transfers are in progress, the LPM will be rejected (NYET response). Only after control transfers are completed (either with ACK/STALL), LPM will be accepted For Isoch transfers, the host needs to do the wake-up and start the transfer. Device controller will not do remote-wakeup when Isoch endpoints get ready. The device SW needs to keep the GUSB2PHYCFG\\[EnblSlpM\\]
reset in order to keep the PHY clock to be running for keeping track of SOF intervals. When L1 hibernation is enabled, the controller will not do automatic exit for hibernation requests thru L1. This bit is quasi-static, i.e., should not be changed during device operation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DevL1ExitByHw::B1
    }
}
#[doc = "Field `DEV_L1_EXIT_BY_HW` writer - DEV_L1_EXIT_BY_HW"]
pub type DevL1ExitByHwW<'a, REG> = crate::BitWriter<'a, REG, DevL1ExitByHw>;
impl<'a, REG> DevL1ExitByHwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default behaviour, disables device L1 hardware exit logic"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DevL1ExitByHw::B0)
    }
    #[doc = "feature enabled This bit is applicable for device mode (2.0) only. This field enables device controller sending remote wakeup for L1 if the device becomes ready for sending/accepting data when in L1 state. If the host expects the device to send remote wkp signalling to resume after going into L1 in flow controlled state, then this bit can be set to send the remote wake signal automatically when the device controller becomes ready. This HW remote wake feature is applicable only to bulk and interrupt transfers, and not for Isoch/Control When control transfers are in progress, the LPM will be rejected (NYET response). Only after control transfers are completed (either with ACK/STALL), LPM will be accepted For Isoch transfers, the host needs to do the wake-up and start the transfer. Device controller will not do remote-wakeup when Isoch endpoints get ready. The device SW needs to keep the GUSB2PHYCFG\\[EnblSlpM\\]
reset in order to keep the PHY clock to be running for keeping track of SOF intervals. When L1 hibernation is enabled, the controller will not do automatic exit for hibernation requests thru L1. This bit is quasi-static, i.e., should not be changed during device operation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DevL1ExitByHw::B1)
    }
}
#[doc = "P3_IN_U2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P3InU2 {
    #[doc = "0: Default behaviour, When SuperSpeed link is in U2 , PowerState P2 is attempted on the PIPE Interface."]
    B0 = 0,
    #[doc = "1: When SuperSpeed link is in U2, PowerState P3 is attempted if GUSB3PIPECTL\\[17\\]
is set. Setting this bit enables P3 Power State when the SuperSpeed link is in U2. Another Power Saving option. When setting this bit to 1 to enable P3 in P2, GUSB3PIPECTL\\[27\\]
should be set to 0 to make sure that the U2 exit is attempted in P0."]
    B1 = 1,
}
impl From<P3InU2> for bool {
    #[inline(always)]
    fn from(variant: P3InU2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P3_IN_U2` reader - P3_IN_U2"]
pub type P3InU2R = crate::BitReader<P3InU2>;
impl P3InU2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3InU2 {
        match self.bits {
            false => P3InU2::B0,
            true => P3InU2::B1,
        }
    }
    #[doc = "Default behaviour, When SuperSpeed link is in U2 , PowerState P2 is attempted on the PIPE Interface."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == P3InU2::B0
    }
    #[doc = "When SuperSpeed link is in U2, PowerState P3 is attempted if GUSB3PIPECTL\\[17\\]
is set. Setting this bit enables P3 Power State when the SuperSpeed link is in U2. Another Power Saving option. When setting this bit to 1 to enable P3 in P2, GUSB3PIPECTL\\[27\\]
should be set to 0 to make sure that the U2 exit is attempted in P0."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == P3InU2::B1
    }
}
#[doc = "Field `P3_IN_U2` writer - P3_IN_U2"]
pub type P3InU2W<'a, REG> = crate::BitWriter<'a, REG, P3InU2>;
impl<'a, REG> P3InU2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default behaviour, When SuperSpeed link is in U2 , PowerState P2 is attempted on the PIPE Interface."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(P3InU2::B0)
    }
    #[doc = "When SuperSpeed link is in U2, PowerState P3 is attempted if GUSB3PIPECTL\\[17\\]
is set. Setting this bit enables P3 Power State when the SuperSpeed link is in U2. Another Power Saving option. When setting this bit to 1 to enable P3 in P2, GUSB3PIPECTL\\[27\\]
should be set to 0 to make sure that the U2 exit is attempted in P0."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(P3InU2::B1)
    }
}
#[doc = "DEV_FORCE_20_CLK_FOR_30_CLK\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevForce20ClkFor30Clk {
    #[doc = "0: Default behaviour, Uses 3.0 clock when operating in 2.0 mode"]
    B0 = 0,
    #[doc = "1: Feature enabled This bit is applicable (and to be set) for device mode (DCFG.Speed!= SS) only. In the 3.0 device core, if the core is programmed to operate in 2.0 only (i.e., Device Speed is programmed to 2.0 speeds in DCFG\\[Speed\\]), then setting this bit makes the internal 2.0 (utmi/ulpi) clock to be routed as the 3.0 (pipe) clock. Enabling this feature allows the pipe3 clock to be not-running when forcibily operating in 2.0 device mode. Note: When using this feature, all pipe3 inputs must be in inactive mode, esp. pipe3 clocks not running and pipe3_phystatus_async must be tied to 0. This bit should not be set if the core is programmed to operate in SuperSpeed mode (even when it falls back to 2.0). This bit is quasi-static, i.e., should not be changed during operation."]
    B1 = 1,
}
impl From<DevForce20ClkFor30Clk> for bool {
    #[inline(always)]
    fn from(variant: DevForce20ClkFor30Clk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_FORCE_20_CLK_FOR_30_CLK` reader - DEV_FORCE_20_CLK_FOR_30_CLK"]
pub type DevForce20ClkFor30ClkR = crate::BitReader<DevForce20ClkFor30Clk>;
impl DevForce20ClkFor30ClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevForce20ClkFor30Clk {
        match self.bits {
            false => DevForce20ClkFor30Clk::B0,
            true => DevForce20ClkFor30Clk::B1,
        }
    }
    #[doc = "Default behaviour, Uses 3.0 clock when operating in 2.0 mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DevForce20ClkFor30Clk::B0
    }
    #[doc = "Feature enabled This bit is applicable (and to be set) for device mode (DCFG.Speed!= SS) only. In the 3.0 device core, if the core is programmed to operate in 2.0 only (i.e., Device Speed is programmed to 2.0 speeds in DCFG\\[Speed\\]), then setting this bit makes the internal 2.0 (utmi/ulpi) clock to be routed as the 3.0 (pipe) clock. Enabling this feature allows the pipe3 clock to be not-running when forcibily operating in 2.0 device mode. Note: When using this feature, all pipe3 inputs must be in inactive mode, esp. pipe3 clocks not running and pipe3_phystatus_async must be tied to 0. This bit should not be set if the core is programmed to operate in SuperSpeed mode (even when it falls back to 2.0). This bit is quasi-static, i.e., should not be changed during operation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DevForce20ClkFor30Clk::B1
    }
}
#[doc = "Field `DEV_FORCE_20_CLK_FOR_30_CLK` writer - DEV_FORCE_20_CLK_FOR_30_CLK"]
pub type DevForce20ClkFor30ClkW<'a, REG> = crate::BitWriter<'a, REG, DevForce20ClkFor30Clk>;
impl<'a, REG> DevForce20ClkFor30ClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default behaviour, Uses 3.0 clock when operating in 2.0 mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DevForce20ClkFor30Clk::B0)
    }
    #[doc = "Feature enabled This bit is applicable (and to be set) for device mode (DCFG.Speed!= SS) only. In the 3.0 device core, if the core is programmed to operate in 2.0 only (i.e., Device Speed is programmed to 2.0 speeds in DCFG\\[Speed\\]), then setting this bit makes the internal 2.0 (utmi/ulpi) clock to be routed as the 3.0 (pipe) clock. Enabling this feature allows the pipe3 clock to be not-running when forcibily operating in 2.0 device mode. Note: When using this feature, all pipe3 inputs must be in inactive mode, esp. pipe3 clocks not running and pipe3_phystatus_async must be tied to 0. This bit should not be set if the core is programmed to operate in SuperSpeed mode (even when it falls back to 2.0). This bit is quasi-static, i.e., should not be changed during operation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DevForce20ClkFor30Clk::B1)
    }
}
#[doc = "DEV_TRB_OUT_SPR_IND\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevTrbOutSprInd {
    #[doc = "0: Default behaviour, no change in TRB status dword"]
    B0 = 0,
    #[doc = "1: Feature enabled, OUT TRB status indicates Short Packet This bit is applicable for device mode only (and ignored in host mode). If the device application (SW/HW) wants to know if a short packet was received for an OUT in the TRB status itself, then this feature can be enabled, so that a bit is set in the TRB writeback in the buf_size dword. Bit\\[26\\]
- SPR of the trbstatus, RSVD, SPR,PCM1, bufsize dword will be set during an OUT transfer TRB write back if this is the last TRB used for that transfer descriptor. This bit is quasi-static, i.e., should not be changed during device operation."]
    B1 = 1,
}
impl From<DevTrbOutSprInd> for bool {
    #[inline(always)]
    fn from(variant: DevTrbOutSprInd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_TRB_OUT_SPR_IND` reader - DEV_TRB_OUT_SPR_IND"]
pub type DevTrbOutSprIndR = crate::BitReader<DevTrbOutSprInd>;
impl DevTrbOutSprIndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevTrbOutSprInd {
        match self.bits {
            false => DevTrbOutSprInd::B0,
            true => DevTrbOutSprInd::B1,
        }
    }
    #[doc = "Default behaviour, no change in TRB status dword"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DevTrbOutSprInd::B0
    }
    #[doc = "Feature enabled, OUT TRB status indicates Short Packet This bit is applicable for device mode only (and ignored in host mode). If the device application (SW/HW) wants to know if a short packet was received for an OUT in the TRB status itself, then this feature can be enabled, so that a bit is set in the TRB writeback in the buf_size dword. Bit\\[26\\]
- SPR of the trbstatus, RSVD, SPR,PCM1, bufsize dword will be set during an OUT transfer TRB write back if this is the last TRB used for that transfer descriptor. This bit is quasi-static, i.e., should not be changed during device operation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DevTrbOutSprInd::B1
    }
}
#[doc = "Field `DEV_TRB_OUT_SPR_IND` writer - DEV_TRB_OUT_SPR_IND"]
pub type DevTrbOutSprIndW<'a, REG> = crate::BitWriter<'a, REG, DevTrbOutSprInd>;
impl<'a, REG> DevTrbOutSprIndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default behaviour, no change in TRB status dword"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DevTrbOutSprInd::B0)
    }
    #[doc = "Feature enabled, OUT TRB status indicates Short Packet This bit is applicable for device mode only (and ignored in host mode). If the device application (SW/HW) wants to know if a short packet was received for an OUT in the TRB status itself, then this feature can be enabled, so that a bit is set in the TRB writeback in the buf_size dword. Bit\\[26\\]
- SPR of the trbstatus, RSVD, SPR,PCM1, bufsize dword will be set during an OUT transfer TRB write back if this is the last TRB used for that transfer descriptor. This bit is quasi-static, i.e., should not be changed during device operation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DevTrbOutSprInd::B1)
    }
}
#[doc = "TX_IPGAP_LINECHECK_DIS\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxIpgapLinecheckDis {
    #[doc = "0: Default behaviour, no change in Linestate check"]
    B0 = 0,
    #[doc = "1: Feature enabled, 2.0 MAC disables Linestate check during HS transmit This bit is applicable for HS operation of u2mac. If this feature is enabled, then the 2.0 mac operating in HS ignores the UTMI/ULPI Linestate during the transmit of a token (during token-to-token and token-to-data IPGAP). When enabled, the controller implements a fixed 40-bit TxEndDelay after the packet is given on UTMI and ignores the Linestate during this time. This fetaure is applicable only in HS mode of operation. Device mode: If GUCTL1.TX_IPGAP_LINECHECK_DIS is set, then for device LPM handshake, the core will ignore the linestate after TX and wait for a fixed clocks ( 40 bit times equivalent) after transmiting ACK on utmi. Host mode: If GUCTL1.TX_IPGAP_LINECHECK_DIS is set, then the ipgap between (tkn to tkn/data) is added by 40 bit times of TXENDDELAY, and linestate is ignored during this 40 bit times delay. Enable this bit if the LineState will not reflect the expected line state (J) during transmission. This bit is quasi-static, i.e., should not be changed during device operation."]
    B1 = 1,
}
impl From<TxIpgapLinecheckDis> for bool {
    #[inline(always)]
    fn from(variant: TxIpgapLinecheckDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_IPGAP_LINECHECK_DIS` reader - TX_IPGAP_LINECHECK_DIS"]
pub type TxIpgapLinecheckDisR = crate::BitReader<TxIpgapLinecheckDis>;
impl TxIpgapLinecheckDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxIpgapLinecheckDis {
        match self.bits {
            false => TxIpgapLinecheckDis::B0,
            true => TxIpgapLinecheckDis::B1,
        }
    }
    #[doc = "Default behaviour, no change in Linestate check"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TxIpgapLinecheckDis::B0
    }
    #[doc = "Feature enabled, 2.0 MAC disables Linestate check during HS transmit This bit is applicable for HS operation of u2mac. If this feature is enabled, then the 2.0 mac operating in HS ignores the UTMI/ULPI Linestate during the transmit of a token (during token-to-token and token-to-data IPGAP). When enabled, the controller implements a fixed 40-bit TxEndDelay after the packet is given on UTMI and ignores the Linestate during this time. This fetaure is applicable only in HS mode of operation. Device mode: If GUCTL1.TX_IPGAP_LINECHECK_DIS is set, then for device LPM handshake, the core will ignore the linestate after TX and wait for a fixed clocks ( 40 bit times equivalent) after transmiting ACK on utmi. Host mode: If GUCTL1.TX_IPGAP_LINECHECK_DIS is set, then the ipgap between (tkn to tkn/data) is added by 40 bit times of TXENDDELAY, and linestate is ignored during this 40 bit times delay. Enable this bit if the LineState will not reflect the expected line state (J) during transmission. This bit is quasi-static, i.e., should not be changed during device operation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TxIpgapLinecheckDis::B1
    }
}
#[doc = "Field `TX_IPGAP_LINECHECK_DIS` writer - TX_IPGAP_LINECHECK_DIS"]
pub type TxIpgapLinecheckDisW<'a, REG> = crate::BitWriter<'a, REG, TxIpgapLinecheckDis>;
impl<'a, REG> TxIpgapLinecheckDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default behaviour, no change in Linestate check"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TxIpgapLinecheckDis::B0)
    }
    #[doc = "Feature enabled, 2.0 MAC disables Linestate check during HS transmit This bit is applicable for HS operation of u2mac. If this feature is enabled, then the 2.0 mac operating in HS ignores the UTMI/ULPI Linestate during the transmit of a token (during token-to-token and token-to-data IPGAP). When enabled, the controller implements a fixed 40-bit TxEndDelay after the packet is given on UTMI and ignores the Linestate during this time. This fetaure is applicable only in HS mode of operation. Device mode: If GUCTL1.TX_IPGAP_LINECHECK_DIS is set, then for device LPM handshake, the core will ignore the linestate after TX and wait for a fixed clocks ( 40 bit times equivalent) after transmiting ACK on utmi. Host mode: If GUCTL1.TX_IPGAP_LINECHECK_DIS is set, then the ipgap between (tkn to tkn/data) is added by 40 bit times of TXENDDELAY, and linestate is ignored during this 40 bit times delay. Enable this bit if the LineState will not reflect the expected line state (J) during transmission. This bit is quasi-static, i.e., should not be changed during device operation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TxIpgapLinecheckDis::B1)
    }
}
#[doc = "FILTER_SE0_FSLS_EOP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterSe0FslsEop {
    #[doc = "0: Default behaviour, no change in Linestate check for SE0 detection in FS/LS"]
    B0 = 0,
    #[doc = "1: Feature enabled, FS/LS SE0 is filtered for 2 clocks for detecting EOP This bit is applicable for FS/LS operation. If this feature is enabled, then SE0 on the linestate is validated for 2 consecutive utmi/ulpi clock edges for EOP detection. This feature is applicable only in FS in device mode and FS/LS mode of operation in host mode. Device mode: FS - If GUCTL1.FILTER_SE0_FSLS_EOP is set, then for device LPM hanshake, the core will ignore single SE0 glitch on the linestate during transmit. Only 2 or more SE0 is considered as a valid EOP on FS. Host mode: FS/LS - If GUCTL1.FILTER_SE0_FSLS_EOP is set, then the core will ignore single SE0 glitch on the linestate during transmit. Only 2 or more SE0 is considered as a valid EOP on FS/LS port. Enable this feature if the LineState has SE0 glitches during transmission. This bit is quasi-static, i.e., should not be changed during device operation."]
    B1 = 1,
}
impl From<FilterSe0FslsEop> for bool {
    #[inline(always)]
    fn from(variant: FilterSe0FslsEop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTER_SE0_FSLS_EOP` reader - FILTER_SE0_FSLS_EOP"]
pub type FilterSe0FslsEopR = crate::BitReader<FilterSe0FslsEop>;
impl FilterSe0FslsEopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FilterSe0FslsEop {
        match self.bits {
            false => FilterSe0FslsEop::B0,
            true => FilterSe0FslsEop::B1,
        }
    }
    #[doc = "Default behaviour, no change in Linestate check for SE0 detection in FS/LS"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FilterSe0FslsEop::B0
    }
    #[doc = "Feature enabled, FS/LS SE0 is filtered for 2 clocks for detecting EOP This bit is applicable for FS/LS operation. If this feature is enabled, then SE0 on the linestate is validated for 2 consecutive utmi/ulpi clock edges for EOP detection. This feature is applicable only in FS in device mode and FS/LS mode of operation in host mode. Device mode: FS - If GUCTL1.FILTER_SE0_FSLS_EOP is set, then for device LPM hanshake, the core will ignore single SE0 glitch on the linestate during transmit. Only 2 or more SE0 is considered as a valid EOP on FS. Host mode: FS/LS - If GUCTL1.FILTER_SE0_FSLS_EOP is set, then the core will ignore single SE0 glitch on the linestate during transmit. Only 2 or more SE0 is considered as a valid EOP on FS/LS port. Enable this feature if the LineState has SE0 glitches during transmission. This bit is quasi-static, i.e., should not be changed during device operation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FilterSe0FslsEop::B1
    }
}
#[doc = "Field `FILTER_SE0_FSLS_EOP` writer - FILTER_SE0_FSLS_EOP"]
pub type FilterSe0FslsEopW<'a, REG> = crate::BitWriter<'a, REG, FilterSe0FslsEop>;
impl<'a, REG> FilterSe0FslsEopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default behaviour, no change in Linestate check for SE0 detection in FS/LS"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FilterSe0FslsEop::B0)
    }
    #[doc = "Feature enabled, FS/LS SE0 is filtered for 2 clocks for detecting EOP This bit is applicable for FS/LS operation. If this feature is enabled, then SE0 on the linestate is validated for 2 consecutive utmi/ulpi clock edges for EOP detection. This feature is applicable only in FS in device mode and FS/LS mode of operation in host mode. Device mode: FS - If GUCTL1.FILTER_SE0_FSLS_EOP is set, then for device LPM hanshake, the core will ignore single SE0 glitch on the linestate during transmit. Only 2 or more SE0 is considered as a valid EOP on FS. Host mode: FS/LS - If GUCTL1.FILTER_SE0_FSLS_EOP is set, then the core will ignore single SE0 glitch on the linestate during transmit. Only 2 or more SE0 is considered as a valid EOP on FS/LS port. Enable this feature if the LineState has SE0 glitches during transmission. This bit is quasi-static, i.e., should not be changed during device operation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FilterSe0FslsEop::B1)
    }
}
impl R {
    #[doc = "Bit 0 - LOA_FILTER_EN\n\nIf this bit is set, the USB 2.0 port babble is checked at least three\n\nconsecutive times before the port is disabled. This prevents false\n\ntriggering of the babble condition when using low quality cables.\n\nNote: This bit is valid only in host mode."]
    #[inline(always)]
    pub fn loa_filter_en(&self) -> LoaFilterEnR {
        LoaFilterEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OVRLD_L1_SUSP_COM\n\nIf this bit is set, the utmi_l1_suspend_com_n is overloaded with\n\nthe utmi_sleep_n signal. This bit is usually set if the PHY stops\n\nthe port clock during L1 sleep condition."]
    #[inline(always)]
    pub fn ovrld_l1_susp_com(&self) -> OvrldL1SuspComR {
        OvrldL1SuspComR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host Parameter Check Disable\n\nWhen this bit is set to 0 (by default), the xHC checks that the\n\ninput slot/EP context fields comply to the xHCI Specification.\n\nUpon detection of a parameter error during command execution,\n\nthe xHC generates an event TRB with completion code indicating\n\nPARAMETER ERROR.\n\nWhen the bit is set to 1, the xHC does not perform parameter\n\nchecks and does not generate PARAMETER ERROR completion\n\ncode."]
    #[inline(always)]
    pub fn hc_parchk_disable(&self) -> HcParchkDisableR {
        HcParchkDisableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Host ELD Enable\n\nWhen this bit is set to 1, it enables the Exit Latency Delta (ELD)\n\nsupport defined in the xHCI 1.0 Errata.\n\nThis bit is used only in the host mode. This bit has to be set to 1\n\nin Host mode."]
    #[inline(always)]
    pub fn hc_errata_enable(&self) -> HcErrataEnableR {
        HcErrataEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - L1_SUSP_THRLD_FOR_HOST\n\nThis field is effective only when the\n\nL1_SUSP_THRLD_EN_FOR_HOST bit is set to 1. For more details,\n\nrefer to the description of the L1_SUSP_THRLD_EN_FOR_HOST\n\nbit."]
    #[inline(always)]
    pub fn l1_susp_thrld_for_host(&self) -> L1SuspThrldForHostR {
        L1SuspThrldForHostR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - L1_SUSP_THRLD_EN_FOR_HOST\n\nThis bit is used only in host mode.\n\nThe host controller asserts the utmi_l1_suspend_n and\n\nutmi_sleep_n output signals (see LPM Interface Signals table in\n\nthe Databook) as follows:\n\nThe controller asserts the utmi_l1_suspend_n signal to put the\n\nPHY into deep low-power mode in L1 when both of the following\n\nare true:\n\nThe HIRD/BESL value used is greater than or equal to the value\n\nin L1_SUSP_THRLD_FOR_HOST field.\n\nThe L1_SUSP_THRLD_EN_FOR_HOST bit is set to 1'b1. The\n\ncontroller asserts utmi_sleep_n on L1 when one of the following\n\nis true:\n\nThe HIRD/BESL value used is less than the value in\n\nL1_SUSP_THRLD_FOR_HOST field.\n\nThe L1_SUSP_THRLD_EN_FOR_HOST bit is set to 1'b0."]
    #[inline(always)]
    pub fn l1_susp_thrld_en_for_host(&self) -> L1SuspThrldEnForHostR {
        L1SuspThrldEnForHostR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - PARKMODE_DISABLE_FSLS\n\nThis bit is used only in host mode, and is for debug purpose only.\n\nWhen this bit is set to 1 all FS/LS bus instances in park mode\n\ndisabled."]
    #[inline(always)]
    pub fn parkmode_disable_fsls(&self) -> ParkmodeDisableFslsR {
        ParkmodeDisableFslsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PARKMODE_DISABLE_HS\n\nThis bit is used only in host mode.\n\nWhen this bit is set to 1 all HS bus instances park mode are\n\ndisabled.\n\nTo improve performance in park mode, the xHCI scheduler\n\nqueues in three requests of 4 packets each for High Speed\n\nasynchronous endpoints in a micro-frame. But if a device is slow\n\nand if it NAKs more than 3 times, then it is rescheduled only in\n\nthe next micro-frame. This could decrease the performance of a\n\nslow device even further.\n\nIn a few high speed devices (such as Sandisk Cruzer Blade 4GB\n\nVID:1921, PID:21863 and Flex Drive VID:3744, PID:8552) when\n\nan IN request is sent within 900ns of the ACK of the previous\n\npacket, these devices send a NAK. When connected to these\n\ndevices, if required, the software can disable the park mode if\n\nyou see performance drop in your system. When park mode is\n\ndisabled, pipelining of multiple packet is disabled and instead one\n\npacket at a time is requested by the scheduler. This allows up to\n\n12 NAKs in a micro-frame and improves performance of these\n\nslow devices."]
    #[inline(always)]
    pub fn parkmode_disable_hs(&self) -> ParkmodeDisableHsR {
        ParkmodeDisableHsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PARKMODE_DISABLE_SS\n\nThis bit is used only in host mode, and is for debug purpose only.\n\nWhen this bit is set to 1 all SS bus instances in park mode are\n\ndisabled."]
    #[inline(always)]
    pub fn parkmode_disable_ss(&self) -> ParkmodeDisableSsR {
        ParkmodeDisableSsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NAK_PER_ENH_HS"]
    #[inline(always)]
    pub fn nak_per_enh_hs(&self) -> NakPerEnhHsR {
        NakPerEnhHsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NAK_PER_ENH_FS"]
    #[inline(always)]
    pub fn nak_per_enh_fs(&self) -> NakPerEnhFsR {
        NakPerEnhFsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DEV_LSP_TAIL_LOCK_DIS"]
    #[inline(always)]
    pub fn dev_lsp_tail_lock_dis(&self) -> DevLspTailLockDisR {
        DevLspTailLockDisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - IP_GAP_ADD_ON\n\nThis register field is used to add on to the default inter packet\n\ngap setting in the USB 2.0 MAC. This should be programmed to a\n\nnon zero value only in case where you need to increase the\n\ndefault inter packet delay calculations in the USB 2.0 MAC\n\nmodule DWC_usb3_u2mac.v"]
    #[inline(always)]
    pub fn ip_gap_add_on(&self) -> IpGapAddOnR {
        IpGapAddOnR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - DEV_L1_EXIT_BY_HW"]
    #[inline(always)]
    pub fn dev_l1_exit_by_hw(&self) -> DevL1ExitByHwR {
        DevL1ExitByHwR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - P3_IN_U2"]
    #[inline(always)]
    pub fn p3_in_u2(&self) -> P3InU2R {
        P3InU2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DEV_FORCE_20_CLK_FOR_30_CLK"]
    #[inline(always)]
    pub fn dev_force_20_clk_for_30_clk(&self) -> DevForce20ClkFor30ClkR {
        DevForce20ClkFor30ClkR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DEV_TRB_OUT_SPR_IND"]
    #[inline(always)]
    pub fn dev_trb_out_spr_ind(&self) -> DevTrbOutSprIndR {
        DevTrbOutSprIndR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TX_IPGAP_LINECHECK_DIS"]
    #[inline(always)]
    pub fn tx_ipgap_linecheck_dis(&self) -> TxIpgapLinecheckDisR {
        TxIpgapLinecheckDisR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FILTER_SE0_FSLS_EOP"]
    #[inline(always)]
    pub fn filter_se0_fsls_eop(&self) -> FilterSe0FslsEopR {
        FilterSe0FslsEopR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOA_FILTER_EN\n\nIf this bit is set, the USB 2.0 port babble is checked at least three\n\nconsecutive times before the port is disabled. This prevents false\n\ntriggering of the babble condition when using low quality cables.\n\nNote: This bit is valid only in host mode."]
    #[inline(always)]
    #[must_use]
    pub fn loa_filter_en(&mut self) -> LoaFilterEnW<Guctl1Spec> {
        LoaFilterEnW::new(self, 0)
    }
    #[doc = "Bit 1 - OVRLD_L1_SUSP_COM\n\nIf this bit is set, the utmi_l1_suspend_com_n is overloaded with\n\nthe utmi_sleep_n signal. This bit is usually set if the PHY stops\n\nthe port clock during L1 sleep condition."]
    #[inline(always)]
    #[must_use]
    pub fn ovrld_l1_susp_com(&mut self) -> OvrldL1SuspComW<Guctl1Spec> {
        OvrldL1SuspComW::new(self, 1)
    }
    #[doc = "Bit 2 - Host Parameter Check Disable\n\nWhen this bit is set to 0 (by default), the xHC checks that the\n\ninput slot/EP context fields comply to the xHCI Specification.\n\nUpon detection of a parameter error during command execution,\n\nthe xHC generates an event TRB with completion code indicating\n\nPARAMETER ERROR.\n\nWhen the bit is set to 1, the xHC does not perform parameter\n\nchecks and does not generate PARAMETER ERROR completion\n\ncode."]
    #[inline(always)]
    #[must_use]
    pub fn hc_parchk_disable(&mut self) -> HcParchkDisableW<Guctl1Spec> {
        HcParchkDisableW::new(self, 2)
    }
    #[doc = "Bit 3 - Host ELD Enable\n\nWhen this bit is set to 1, it enables the Exit Latency Delta (ELD)\n\nsupport defined in the xHCI 1.0 Errata.\n\nThis bit is used only in the host mode. This bit has to be set to 1\n\nin Host mode."]
    #[inline(always)]
    #[must_use]
    pub fn hc_errata_enable(&mut self) -> HcErrataEnableW<Guctl1Spec> {
        HcErrataEnableW::new(self, 3)
    }
    #[doc = "Bits 4:7 - L1_SUSP_THRLD_FOR_HOST\n\nThis field is effective only when the\n\nL1_SUSP_THRLD_EN_FOR_HOST bit is set to 1. For more details,\n\nrefer to the description of the L1_SUSP_THRLD_EN_FOR_HOST\n\nbit."]
    #[inline(always)]
    #[must_use]
    pub fn l1_susp_thrld_for_host(&mut self) -> L1SuspThrldForHostW<Guctl1Spec> {
        L1SuspThrldForHostW::new(self, 4)
    }
    #[doc = "Bit 8 - L1_SUSP_THRLD_EN_FOR_HOST\n\nThis bit is used only in host mode.\n\nThe host controller asserts the utmi_l1_suspend_n and\n\nutmi_sleep_n output signals (see LPM Interface Signals table in\n\nthe Databook) as follows:\n\nThe controller asserts the utmi_l1_suspend_n signal to put the\n\nPHY into deep low-power mode in L1 when both of the following\n\nare true:\n\nThe HIRD/BESL value used is greater than or equal to the value\n\nin L1_SUSP_THRLD_FOR_HOST field.\n\nThe L1_SUSP_THRLD_EN_FOR_HOST bit is set to 1'b1. The\n\ncontroller asserts utmi_sleep_n on L1 when one of the following\n\nis true:\n\nThe HIRD/BESL value used is less than the value in\n\nL1_SUSP_THRLD_FOR_HOST field.\n\nThe L1_SUSP_THRLD_EN_FOR_HOST bit is set to 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn l1_susp_thrld_en_for_host(&mut self) -> L1SuspThrldEnForHostW<Guctl1Spec> {
        L1SuspThrldEnForHostW::new(self, 8)
    }
    #[doc = "Bit 15 - PARKMODE_DISABLE_FSLS\n\nThis bit is used only in host mode, and is for debug purpose only.\n\nWhen this bit is set to 1 all FS/LS bus instances in park mode\n\ndisabled."]
    #[inline(always)]
    #[must_use]
    pub fn parkmode_disable_fsls(&mut self) -> ParkmodeDisableFslsW<Guctl1Spec> {
        ParkmodeDisableFslsW::new(self, 15)
    }
    #[doc = "Bit 16 - PARKMODE_DISABLE_HS\n\nThis bit is used only in host mode.\n\nWhen this bit is set to 1 all HS bus instances park mode are\n\ndisabled.\n\nTo improve performance in park mode, the xHCI scheduler\n\nqueues in three requests of 4 packets each for High Speed\n\nasynchronous endpoints in a micro-frame. But if a device is slow\n\nand if it NAKs more than 3 times, then it is rescheduled only in\n\nthe next micro-frame. This could decrease the performance of a\n\nslow device even further.\n\nIn a few high speed devices (such as Sandisk Cruzer Blade 4GB\n\nVID:1921, PID:21863 and Flex Drive VID:3744, PID:8552) when\n\nan IN request is sent within 900ns of the ACK of the previous\n\npacket, these devices send a NAK. When connected to these\n\ndevices, if required, the software can disable the park mode if\n\nyou see performance drop in your system. When park mode is\n\ndisabled, pipelining of multiple packet is disabled and instead one\n\npacket at a time is requested by the scheduler. This allows up to\n\n12 NAKs in a micro-frame and improves performance of these\n\nslow devices."]
    #[inline(always)]
    #[must_use]
    pub fn parkmode_disable_hs(&mut self) -> ParkmodeDisableHsW<Guctl1Spec> {
        ParkmodeDisableHsW::new(self, 16)
    }
    #[doc = "Bit 17 - PARKMODE_DISABLE_SS\n\nThis bit is used only in host mode, and is for debug purpose only.\n\nWhen this bit is set to 1 all SS bus instances in park mode are\n\ndisabled."]
    #[inline(always)]
    #[must_use]
    pub fn parkmode_disable_ss(&mut self) -> ParkmodeDisableSsW<Guctl1Spec> {
        ParkmodeDisableSsW::new(self, 17)
    }
    #[doc = "Bit 18 - NAK_PER_ENH_HS"]
    #[inline(always)]
    #[must_use]
    pub fn nak_per_enh_hs(&mut self) -> NakPerEnhHsW<Guctl1Spec> {
        NakPerEnhHsW::new(self, 18)
    }
    #[doc = "Bit 19 - NAK_PER_ENH_FS"]
    #[inline(always)]
    #[must_use]
    pub fn nak_per_enh_fs(&mut self) -> NakPerEnhFsW<Guctl1Spec> {
        NakPerEnhFsW::new(self, 19)
    }
    #[doc = "Bit 20 - DEV_LSP_TAIL_LOCK_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn dev_lsp_tail_lock_dis(&mut self) -> DevLspTailLockDisW<Guctl1Spec> {
        DevLspTailLockDisW::new(self, 20)
    }
    #[doc = "Bits 21:23 - IP_GAP_ADD_ON\n\nThis register field is used to add on to the default inter packet\n\ngap setting in the USB 2.0 MAC. This should be programmed to a\n\nnon zero value only in case where you need to increase the\n\ndefault inter packet delay calculations in the USB 2.0 MAC\n\nmodule DWC_usb3_u2mac.v"]
    #[inline(always)]
    #[must_use]
    pub fn ip_gap_add_on(&mut self) -> IpGapAddOnW<Guctl1Spec> {
        IpGapAddOnW::new(self, 21)
    }
    #[doc = "Bit 24 - DEV_L1_EXIT_BY_HW"]
    #[inline(always)]
    #[must_use]
    pub fn dev_l1_exit_by_hw(&mut self) -> DevL1ExitByHwW<Guctl1Spec> {
        DevL1ExitByHwW::new(self, 24)
    }
    #[doc = "Bit 25 - P3_IN_U2"]
    #[inline(always)]
    #[must_use]
    pub fn p3_in_u2(&mut self) -> P3InU2W<Guctl1Spec> {
        P3InU2W::new(self, 25)
    }
    #[doc = "Bit 26 - DEV_FORCE_20_CLK_FOR_30_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dev_force_20_clk_for_30_clk(&mut self) -> DevForce20ClkFor30ClkW<Guctl1Spec> {
        DevForce20ClkFor30ClkW::new(self, 26)
    }
    #[doc = "Bit 27 - DEV_TRB_OUT_SPR_IND"]
    #[inline(always)]
    #[must_use]
    pub fn dev_trb_out_spr_ind(&mut self) -> DevTrbOutSprIndW<Guctl1Spec> {
        DevTrbOutSprIndW::new(self, 27)
    }
    #[doc = "Bit 28 - TX_IPGAP_LINECHECK_DIS"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ipgap_linecheck_dis(&mut self) -> TxIpgapLinecheckDisW<Guctl1Spec> {
        TxIpgapLinecheckDisW::new(self, 28)
    }
    #[doc = "Bit 29 - FILTER_SE0_FSLS_EOP"]
    #[inline(always)]
    #[must_use]
    pub fn filter_se0_fsls_eop(&mut self) -> FilterSe0FslsEopW<Guctl1Spec> {
        FilterSe0FslsEopW::new(self, 29)
    }
}
#[doc = "Global User Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Guctl1Spec;
impl crate::RegisterSpec for Guctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`guctl1::R`](R) reader structure"]
impl crate::Readable for Guctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`guctl1::W`](W) writer structure"]
impl crate::Writable for Guctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUCTL1 to value 0x0004_018a"]
impl crate::Resettable for Guctl1Spec {
    const RESET_VALUE: u32 = 0x0004_018a;
}
