#[doc = "Register `HOSTCTRL2` reader"]
pub type R = crate::R<Hostctrl2Spec>;
#[doc = "Register `HOSTCTRL2` writer"]
pub type W = crate::W<Hostctrl2Spec>;
#[doc = "UHS Mode Select.\n\nThis field is used to select one of UHS-I modes and effective\n\nwhen 1.8V Signaling Enable is set to 1.\n\nIf Preset Value Enable in the Host Control 2 register is set to 1,\n\nHost Controller sets SDCLK Frequency Select, Clock Generator\n\nSelect in the Clock Control register and Driver Strength Select\n\naccording to Preset Value registers. In this case, one of preset\n\nvalue registers is selected by this field. Host Driver needs to reset\n\nSD Clock Enable before changing this field to avoid generating\n\nclock glitch. After setting this field, Host Driver sets SD Clock\n\nEnable again.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uhsmodeselect {
    #[doc = "0: SDR12"]
    H0 = 0,
    #[doc = "1: SDR25"]
    H1 = 1,
    #[doc = "2: SDR50"]
    H2 = 2,
    #[doc = "3: SDR104"]
    H3 = 3,
    #[doc = "4: DDR50"]
    H4 = 4,
    #[doc = "5: HS400 others: Reserved When SDR50, SDR104 or DDR50 is selected for SDIO card, interrupt detection at the block gap shall not be used. Read Wait timing is changed for these modes. Refer to the SDIO Specification Version 3.00 for more detail."]
    H5 = 5,
}
impl From<Uhsmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Uhsmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uhsmodeselect {
    type Ux = u8;
}
#[doc = "Field `UHSMODESELECT` reader - UHS Mode Select.\n\nThis field is used to select one of UHS-I modes and effective\n\nwhen 1.8V Signaling Enable is set to 1.\n\nIf Preset Value Enable in the Host Control 2 register is set to 1,\n\nHost Controller sets SDCLK Frequency Select, Clock Generator\n\nSelect in the Clock Control register and Driver Strength Select\n\naccording to Preset Value registers. In this case, one of preset\n\nvalue registers is selected by this field. Host Driver needs to reset\n\nSD Clock Enable before changing this field to avoid generating\n\nclock glitch. After setting this field, Host Driver sets SD Clock\n\nEnable again."]
pub type UhsmodeselectR = crate::FieldReader<Uhsmodeselect>;
impl UhsmodeselectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uhsmodeselect> {
        match self.bits {
            0 => Some(Uhsmodeselect::H0),
            1 => Some(Uhsmodeselect::H1),
            2 => Some(Uhsmodeselect::H2),
            3 => Some(Uhsmodeselect::H3),
            4 => Some(Uhsmodeselect::H4),
            5 => Some(Uhsmodeselect::H5),
            _ => None,
        }
    }
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Uhsmodeselect::H0
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Uhsmodeselect::H1
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == Uhsmodeselect::H2
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == Uhsmodeselect::H3
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn is_h4(&self) -> bool {
        *self == Uhsmodeselect::H4
    }
    #[doc = "HS400 others: Reserved When SDR50, SDR104 or DDR50 is selected for SDIO card, interrupt detection at the block gap shall not be used. Read Wait timing is changed for these modes. Refer to the SDIO Specification Version 3.00 for more detail."]
    #[inline(always)]
    pub fn is_h5(&self) -> bool {
        *self == Uhsmodeselect::H5
    }
}
#[doc = "Field `UHSMODESELECT` writer - UHS Mode Select.\n\nThis field is used to select one of UHS-I modes and effective\n\nwhen 1.8V Signaling Enable is set to 1.\n\nIf Preset Value Enable in the Host Control 2 register is set to 1,\n\nHost Controller sets SDCLK Frequency Select, Clock Generator\n\nSelect in the Clock Control register and Driver Strength Select\n\naccording to Preset Value registers. In this case, one of preset\n\nvalue registers is selected by this field. Host Driver needs to reset\n\nSD Clock Enable before changing this field to avoid generating\n\nclock glitch. After setting this field, Host Driver sets SD Clock\n\nEnable again."]
pub type UhsmodeselectW<'a, REG> = crate::FieldWriter<'a, REG, 3, Uhsmodeselect>;
impl<'a, REG> UhsmodeselectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodeselect::H0)
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodeselect::H1)
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodeselect::H2)
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn h3(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodeselect::H3)
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn h4(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodeselect::H4)
    }
    #[doc = "HS400 others: Reserved When SDR50, SDR104 or DDR50 is selected for SDIO card, interrupt detection at the block gap shall not be used. Read Wait timing is changed for these modes. Refer to the SDIO Specification Version 3.00 for more detail."]
    #[inline(always)]
    pub fn h5(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodeselect::H5)
    }
}
#[doc = "This bit is set to 1 to start tuning procedure and automatically\n\ncleared when tuning procedure is completed. The result of tuning\n\nis indicated to Sampling Clock Select. Tuning procedure is\n\naborted by writing 0 for more detail about tuning procedure.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Executetuning {
    #[doc = "1: Execute Tuning"]
    B1 = 1,
    #[doc = "0: Not Tuned or Tuning Completed"]
    B0 = 0,
}
impl From<Executetuning> for bool {
    #[inline(always)]
    fn from(variant: Executetuning) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXECUTETUNING` reader - This bit is set to 1 to start tuning procedure and automatically\n\ncleared when tuning procedure is completed. The result of tuning\n\nis indicated to Sampling Clock Select. Tuning procedure is\n\naborted by writing 0 for more detail about tuning procedure."]
pub type ExecutetuningR = crate::BitReader<Executetuning>;
impl ExecutetuningR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Executetuning {
        match self.bits {
            true => Executetuning::B1,
            false => Executetuning::B0,
        }
    }
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Executetuning::B1
    }
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Executetuning::B0
    }
}
#[doc = "Field `EXECUTETUNING` writer - This bit is set to 1 to start tuning procedure and automatically\n\ncleared when tuning procedure is completed. The result of tuning\n\nis indicated to Sampling Clock Select. Tuning procedure is\n\naborted by writing 0 for more detail about tuning procedure."]
pub type ExecutetuningW<'a, REG> = crate::BitWriter<'a, REG, Executetuning>;
impl<'a, REG> ExecutetuningW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Execute Tuning"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Executetuning::B1)
    }
    #[doc = "Not Tuned or Tuning Completed"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Executetuning::B0)
    }
}
#[doc = "Sampling Clock Select\n\nThis bit is set by tuning procedure when Execute Tuning is\n\ncleared. Writing 1 to this bit is meaningless and ignored. Setting\n\n1 means that tuning is completed successfully and setting 0\n\nmeans that tuning is failed. Host Controller uses this bit to select\n\nsampling clock to receive CMD and DAT. This bit is cleared by\n\nwriting 0. Change of this bit is not allowed while the Host\n\nController is receiving response or a read data block.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Samplingclockselect {
    #[doc = "1: Tuned clock is used to sample data"]
    B1 = 1,
    #[doc = "0: Fixed clock is used to sample data"]
    B0 = 0,
}
impl From<Samplingclockselect> for bool {
    #[inline(always)]
    fn from(variant: Samplingclockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLINGCLOCKSELECT` reader - Sampling Clock Select\n\nThis bit is set by tuning procedure when Execute Tuning is\n\ncleared. Writing 1 to this bit is meaningless and ignored. Setting\n\n1 means that tuning is completed successfully and setting 0\n\nmeans that tuning is failed. Host Controller uses this bit to select\n\nsampling clock to receive CMD and DAT. This bit is cleared by\n\nwriting 0. Change of this bit is not allowed while the Host\n\nController is receiving response or a read data block."]
pub type SamplingclockselectR = crate::BitReader<Samplingclockselect>;
impl SamplingclockselectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Samplingclockselect {
        match self.bits {
            true => Samplingclockselect::B1,
            false => Samplingclockselect::B0,
        }
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Samplingclockselect::B1
    }
    #[doc = "Fixed clock is used to sample data"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Samplingclockselect::B0
    }
}
#[doc = "Field `SAMPLINGCLOCKSELECT` writer - Sampling Clock Select\n\nThis bit is set by tuning procedure when Execute Tuning is\n\ncleared. Writing 1 to this bit is meaningless and ignored. Setting\n\n1 means that tuning is completed successfully and setting 0\n\nmeans that tuning is failed. Host Controller uses this bit to select\n\nsampling clock to receive CMD and DAT. This bit is cleared by\n\nwriting 0. Change of this bit is not allowed while the Host\n\nController is receiving response or a read data block."]
pub type SamplingclockselectW<'a, REG> = crate::BitWriter<'a, REG, Samplingclockselect>;
impl<'a, REG> SamplingclockselectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Samplingclockselect::B1)
    }
    #[doc = "Fixed clock is used to sample data"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Samplingclockselect::B0)
    }
}
#[doc = "Asynchronous Interrupt Enable\n\nThis bit can be set to 1 if a card support asynchronous interrupt\n\nand Asynchronous Interrupt Support is set to 1 in the Capabilities\n\nregister. Asynchronous interrupt is effective when DAT\\[1\\]\n\ninterrupt is used in 4-bit SD mode(and zero is set to Interrupt Pin\n\nSelect in the Shared Bus Control register). If this bit is set to 1,\n\nthe Host Driver can stop the SDCLK during asynchronous\n\ninterrupt period to save power. During this period, the Host\n\nController continues to deliver CardInterrupt to the host when it\n\nis asserted by the card.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asyninten {
    #[doc = "1: Enabled"]
    B1 = 1,
    #[doc = "0: Disabled"]
    B0 = 0,
}
impl From<Asyninten> for bool {
    #[inline(always)]
    fn from(variant: Asyninten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASYNINTEN` reader - Asynchronous Interrupt Enable\n\nThis bit can be set to 1 if a card support asynchronous interrupt\n\nand Asynchronous Interrupt Support is set to 1 in the Capabilities\n\nregister. Asynchronous interrupt is effective when DAT\\[1\\]\n\ninterrupt is used in 4-bit SD mode(and zero is set to Interrupt Pin\n\nSelect in the Shared Bus Control register). If this bit is set to 1,\n\nthe Host Driver can stop the SDCLK during asynchronous\n\ninterrupt period to save power. During this period, the Host\n\nController continues to deliver CardInterrupt to the host when it\n\nis asserted by the card."]
pub type AsynintenR = crate::BitReader<Asyninten>;
impl AsynintenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asyninten {
        match self.bits {
            true => Asyninten::B1,
            false => Asyninten::B0,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Asyninten::B1
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Asyninten::B0
    }
}
#[doc = "Field `ASYNINTEN` writer - Asynchronous Interrupt Enable\n\nThis bit can be set to 1 if a card support asynchronous interrupt\n\nand Asynchronous Interrupt Support is set to 1 in the Capabilities\n\nregister. Asynchronous interrupt is effective when DAT\\[1\\]\n\ninterrupt is used in 4-bit SD mode(and zero is set to Interrupt Pin\n\nSelect in the Shared Bus Control register). If this bit is set to 1,\n\nthe Host Driver can stop the SDCLK during asynchronous\n\ninterrupt period to save power. During this period, the Host\n\nController continues to deliver CardInterrupt to the host when it\n\nis asserted by the card."]
pub type AsynintenW<'a, REG> = crate::BitWriter<'a, REG, Asyninten>;
impl<'a, REG> AsynintenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Asyninten::B1)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Asyninten::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Presetvalueenable {
    #[doc = "1: Automatic Selection by Preset Value are Enabled"]
    B1 = 1,
    #[doc = "0: SDCLK and Driver Strength are controlled by Host Driver As the operating SDCLK frequency and I/O driver strength depend on the Host System implementation, it is difficult to determine these parameters in the Standard Host Driver. When Preset Value Enable is set to automatic. This bit enablesthe functions defined in the Preset Value registers. If this bit is set to 0, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Driver. If this bit is set to 1, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Controller as specified in the Preset Value registers."]
    B0 = 0,
}
impl From<Presetvalueenable> for bool {
    #[inline(always)]
    fn from(variant: Presetvalueenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRESETVALUEENABLE` reader - "]
pub type PresetvalueenableR = crate::BitReader<Presetvalueenable>;
impl PresetvalueenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Presetvalueenable {
        match self.bits {
            true => Presetvalueenable::B1,
            false => Presetvalueenable::B0,
        }
    }
    #[doc = "Automatic Selection by Preset Value are Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Presetvalueenable::B1
    }
    #[doc = "SDCLK and Driver Strength are controlled by Host Driver As the operating SDCLK frequency and I/O driver strength depend on the Host System implementation, it is difficult to determine these parameters in the Standard Host Driver. When Preset Value Enable is set to automatic. This bit enablesthe functions defined in the Preset Value registers. If this bit is set to 0, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Driver. If this bit is set to 1, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Controller as specified in the Preset Value registers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Presetvalueenable::B0
    }
}
#[doc = "Field `PRESETVALUEENABLE` writer - "]
pub type PresetvalueenableW<'a, REG> = crate::BitWriter<'a, REG, Presetvalueenable>;
impl<'a, REG> PresetvalueenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic Selection by Preset Value are Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Presetvalueenable::B1)
    }
    #[doc = "SDCLK and Driver Strength are controlled by Host Driver As the operating SDCLK frequency and I/O driver strength depend on the Host System implementation, it is difficult to determine these parameters in the Standard Host Driver. When Preset Value Enable is set to automatic. This bit enablesthe functions defined in the Preset Value registers. If this bit is set to 0, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Driver. If this bit is set to 1, SDCLK Frequency Select, Clock Generator Select in the Clock Control register and Driver Strength Select in Host Control 2 register are set by Host Controller as specified in the Preset Value registers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Presetvalueenable::B0)
    }
}
impl R {
    #[doc = "Bits 0:2 - UHS Mode Select.\n\nThis field is used to select one of UHS-I modes and effective\n\nwhen 1.8V Signaling Enable is set to 1.\n\nIf Preset Value Enable in the Host Control 2 register is set to 1,\n\nHost Controller sets SDCLK Frequency Select, Clock Generator\n\nSelect in the Clock Control register and Driver Strength Select\n\naccording to Preset Value registers. In this case, one of preset\n\nvalue registers is selected by this field. Host Driver needs to reset\n\nSD Clock Enable before changing this field to avoid generating\n\nclock glitch. After setting this field, Host Driver sets SD Clock\n\nEnable again."]
    #[inline(always)]
    pub fn uhsmodeselect(&self) -> UhsmodeselectR {
        UhsmodeselectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - This bit is set to 1 to start tuning procedure and automatically\n\ncleared when tuning procedure is completed. The result of tuning\n\nis indicated to Sampling Clock Select. Tuning procedure is\n\naborted by writing 0 for more detail about tuning procedure."]
    #[inline(always)]
    pub fn executetuning(&self) -> ExecutetuningR {
        ExecutetuningR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sampling Clock Select\n\nThis bit is set by tuning procedure when Execute Tuning is\n\ncleared. Writing 1 to this bit is meaningless and ignored. Setting\n\n1 means that tuning is completed successfully and setting 0\n\nmeans that tuning is failed. Host Controller uses this bit to select\n\nsampling clock to receive CMD and DAT. This bit is cleared by\n\nwriting 0. Change of this bit is not allowed while the Host\n\nController is receiving response or a read data block."]
    #[inline(always)]
    pub fn samplingclockselect(&self) -> SamplingclockselectR {
        SamplingclockselectR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable\n\nThis bit can be set to 1 if a card support asynchronous interrupt\n\nand Asynchronous Interrupt Support is set to 1 in the Capabilities\n\nregister. Asynchronous interrupt is effective when DAT\\[1\\]\n\ninterrupt is used in 4-bit SD mode(and zero is set to Interrupt Pin\n\nSelect in the Shared Bus Control register). If this bit is set to 1,\n\nthe Host Driver can stop the SDCLK during asynchronous\n\ninterrupt period to save power. During this period, the Host\n\nController continues to deliver CardInterrupt to the host when it\n\nis asserted by the card."]
    #[inline(always)]
    pub fn asyninten(&self) -> AsynintenR {
        AsynintenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn presetvalueenable(&self) -> PresetvalueenableR {
        PresetvalueenableR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - UHS Mode Select.\n\nThis field is used to select one of UHS-I modes and effective\n\nwhen 1.8V Signaling Enable is set to 1.\n\nIf Preset Value Enable in the Host Control 2 register is set to 1,\n\nHost Controller sets SDCLK Frequency Select, Clock Generator\n\nSelect in the Clock Control register and Driver Strength Select\n\naccording to Preset Value registers. In this case, one of preset\n\nvalue registers is selected by this field. Host Driver needs to reset\n\nSD Clock Enable before changing this field to avoid generating\n\nclock glitch. After setting this field, Host Driver sets SD Clock\n\nEnable again."]
    #[inline(always)]
    #[must_use]
    pub fn uhsmodeselect(&mut self) -> UhsmodeselectW<Hostctrl2Spec> {
        UhsmodeselectW::new(self, 0)
    }
    #[doc = "Bit 6 - This bit is set to 1 to start tuning procedure and automatically\n\ncleared when tuning procedure is completed. The result of tuning\n\nis indicated to Sampling Clock Select. Tuning procedure is\n\naborted by writing 0 for more detail about tuning procedure."]
    #[inline(always)]
    #[must_use]
    pub fn executetuning(&mut self) -> ExecutetuningW<Hostctrl2Spec> {
        ExecutetuningW::new(self, 6)
    }
    #[doc = "Bit 7 - Sampling Clock Select\n\nThis bit is set by tuning procedure when Execute Tuning is\n\ncleared. Writing 1 to this bit is meaningless and ignored. Setting\n\n1 means that tuning is completed successfully and setting 0\n\nmeans that tuning is failed. Host Controller uses this bit to select\n\nsampling clock to receive CMD and DAT. This bit is cleared by\n\nwriting 0. Change of this bit is not allowed while the Host\n\nController is receiving response or a read data block."]
    #[inline(always)]
    #[must_use]
    pub fn samplingclockselect(&mut self) -> SamplingclockselectW<Hostctrl2Spec> {
        SamplingclockselectW::new(self, 7)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable\n\nThis bit can be set to 1 if a card support asynchronous interrupt\n\nand Asynchronous Interrupt Support is set to 1 in the Capabilities\n\nregister. Asynchronous interrupt is effective when DAT\\[1\\]\n\ninterrupt is used in 4-bit SD mode(and zero is set to Interrupt Pin\n\nSelect in the Shared Bus Control register). If this bit is set to 1,\n\nthe Host Driver can stop the SDCLK during asynchronous\n\ninterrupt period to save power. During this period, the Host\n\nController continues to deliver CardInterrupt to the host when it\n\nis asserted by the card."]
    #[inline(always)]
    #[must_use]
    pub fn asyninten(&mut self) -> AsynintenW<Hostctrl2Spec> {
        AsynintenW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn presetvalueenable(&mut self) -> PresetvalueenableW<Hostctrl2Spec> {
        PresetvalueenableW::new(self, 15)
    }
}
#[doc = "Host Control 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hostctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hostctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hostctrl2Spec;
impl crate::RegisterSpec for Hostctrl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hostctrl2::R`](R) reader structure"]
impl crate::Readable for Hostctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`hostctrl2::W`](W) writer structure"]
impl crate::Writable for Hostctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HOSTCTRL2 to value 0"]
impl crate::Resettable for Hostctrl2Spec {
    const RESET_VALUE: u16 = 0;
}
