#[doc = "Register `DdrMode` reader"]
pub type R = crate::R<DdrModeSpec>;
#[doc = "Register `DdrMode` writer"]
pub type W = crate::W<DdrModeSpec>;
#[doc = "Field `AUTOPRECHARGE` reader - When set to one, pages are automatically closed after each access,\n\nwhen set to zero, pages are left opened until an access in a different\n\npage occurs"]
pub type AutoprechargeR = crate::BitReader;
#[doc = "Field `AUTOPRECHARGE` writer - When set to one, pages are automatically closed after each access,\n\nwhen set to zero, pages are left opened until an access in a different\n\npage occurs"]
pub type AutoprechargeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASSFILTERING` reader - When register field BypassFiltering is set to 1, arbiter filters are\n\nbypassed and timing register outputs are internally set to an idle\n\nvalue.The field can be useful during DRAM initialization, when\n\ntraining or calibration sequences are performed, and scheduler\n\narbitration is not needed.\n\nWhen the field is set to 0, scheduler arbitration is fully functional,\n\nthis is the functional usage mode.\n\nNOTE: When the field is set to 1, the final arbitration level continues\n\nto elect transactions among those presented to the arbiter. Set field\n\nForceOrder to ensure that transactions are executed in order, for\n\ninstance during DRAM initialization."]
pub type BypassfilteringR = crate::BitReader;
#[doc = "Field `BYPASSFILTERING` writer - When register field BypassFiltering is set to 1, arbiter filters are\n\nbypassed and timing register outputs are internally set to an idle\n\nvalue.The field can be useful during DRAM initialization, when\n\ntraining or calibration sequences are performed, and scheduler\n\narbitration is not needed.\n\nWhen the field is set to 0, scheduler arbitration is fully functional,\n\nthis is the functional usage mode.\n\nNOTE: When the field is set to 1, the final arbitration level continues\n\nto elect transactions among those presented to the arbiter. Set field\n\nForceOrder to ensure that transactions are executed in order, for\n\ninstance during DRAM initialization."]
pub type BypassfilteringW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAWBANK` reader - Register field FawBank indicates the number of banks of a given\n\ndevice involved in the FAW period during which four banks can be\n\nactive.\n\nIt must be set to 0 for 2-bank memories, and 1 for memories with\n\nfour banks or more."]
pub type FawbankR = crate::BitReader;
#[doc = "Field `FAWBANK` writer - Register field FawBank indicates the number of banks of a given\n\ndevice involved in the FAW period during which four banks can be\n\nactive.\n\nIt must be set to 0 for 2-bank memories, and 1 for memories with\n\nfour banks or more."]
pub type FawbankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Register field BurstSize sets the DDR burst size, in bytes, as shown\n\nby the following table.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Burstsize {
    #[doc = "0: 16"]
    B00 = 0,
    #[doc = "1: 32"]
    B01 = 1,
    #[doc = "2: 64"]
    B10 = 2,
    #[doc = "3: 128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
    B11 = 3,
}
impl From<Burstsize> for u8 {
    #[inline(always)]
    fn from(variant: Burstsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Burstsize {
    type Ux = u8;
}
#[doc = "Field `BURSTSIZE` reader - Register field BurstSize sets the DDR burst size, in bytes, as shown\n\nby the following table."]
pub type BurstsizeR = crate::FieldReader<Burstsize>;
impl BurstsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burstsize {
        match self.bits {
            0 => Burstsize::B00,
            1 => Burstsize::B01,
            2 => Burstsize::B10,
            3 => Burstsize::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Burstsize::B00
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Burstsize::B01
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Burstsize::B10
    }
    #[doc = "128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Burstsize::B11
    }
}
#[doc = "Field `BURSTSIZE` writer - Register field BurstSize sets the DDR burst size, in bytes, as shown\n\nby the following table."]
pub type BurstsizeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Burstsize>;
impl<'a, REG> BurstsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsize::B00)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsize::B01)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsize::B10)
    }
    #[doc = "128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsize::B11)
    }
}
#[doc = "Register MwrSize sets LPDDR4 data width, which is used for\n\nmasked-write split\n\ncontrol. The field must be set to non-zero for LPDDR3 memories.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mwrsize {
    #[doc = "1: LPDDR4, 16 bits."]
    B01 = 1,
    #[doc = "2: LPDDR4, 32 bits."]
    B10 = 2,
    #[doc = "3: LPDDR4, 32 bits."]
    B11 = 3,
}
impl From<Mwrsize> for u8 {
    #[inline(always)]
    fn from(variant: Mwrsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mwrsize {
    type Ux = u8;
}
#[doc = "Field `MWRSIZE` reader - Register MwrSize sets LPDDR4 data width, which is used for\n\nmasked-write split\n\ncontrol. The field must be set to non-zero for LPDDR3 memories."]
pub type MwrsizeR = crate::FieldReader<Mwrsize>;
impl MwrsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mwrsize> {
        match self.bits {
            1 => Some(Mwrsize::B01),
            2 => Some(Mwrsize::B10),
            3 => Some(Mwrsize::B11),
            _ => None,
        }
    }
    #[doc = "LPDDR4, 16 bits."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Mwrsize::B01
    }
    #[doc = "LPDDR4, 32 bits."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Mwrsize::B10
    }
    #[doc = "LPDDR4, 32 bits."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Mwrsize::B11
    }
}
#[doc = "Field `MWRSIZE` writer - Register MwrSize sets LPDDR4 data width, which is used for\n\nmasked-write split\n\ncontrol. The field must be set to non-zero for LPDDR3 memories."]
pub type MwrsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mwrsize>;
impl<'a, REG> MwrsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPDDR4, 16 bits."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Mwrsize::B01)
    }
    #[doc = "LPDDR4, 32 bits."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Mwrsize::B10)
    }
    #[doc = "LPDDR4, 32 bits."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Mwrsize::B11)
    }
}
#[doc = "Field `FORCEORDER` reader - When bit n of register field ForceOrder is set to 1, DRAM commands\n\nare executed in the order they arrive at scheduler port n.\n\nWhen field bits are set to 1, and BypassFiltering is also set to 1,\n\ncommand execution order is guaranteed for the corresponding\n\nscheduler port."]
pub type ForceorderR = crate::FieldReader;
#[doc = "Field `FORCEORDER` writer - When bit n of register field ForceOrder is set to 1, DRAM commands\n\nare executed in the order they arrive at scheduler port n.\n\nWhen field bits are set to 1, and BypassFiltering is also set to 1,\n\ncommand execution order is guaranteed for the corresponding\n\nscheduler port."]
pub type ForceorderW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FORCEORDERSTATE` reader - ForceOrderState"]
pub type ForceorderstateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - When set to one, pages are automatically closed after each access,\n\nwhen set to zero, pages are left opened until an access in a different\n\npage occurs"]
    #[inline(always)]
    pub fn autoprecharge(&self) -> AutoprechargeR {
        AutoprechargeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When register field BypassFiltering is set to 1, arbiter filters are\n\nbypassed and timing register outputs are internally set to an idle\n\nvalue.The field can be useful during DRAM initialization, when\n\ntraining or calibration sequences are performed, and scheduler\n\narbitration is not needed.\n\nWhen the field is set to 0, scheduler arbitration is fully functional,\n\nthis is the functional usage mode.\n\nNOTE: When the field is set to 1, the final arbitration level continues\n\nto elect transactions among those presented to the arbiter. Set field\n\nForceOrder to ensure that transactions are executed in order, for\n\ninstance during DRAM initialization."]
    #[inline(always)]
    pub fn bypassfiltering(&self) -> BypassfilteringR {
        BypassfilteringR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register field FawBank indicates the number of banks of a given\n\ndevice involved in the FAW period during which four banks can be\n\nactive.\n\nIt must be set to 0 for 2-bank memories, and 1 for memories with\n\nfour banks or more."]
    #[inline(always)]
    pub fn fawbank(&self) -> FawbankR {
        FawbankR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Register field BurstSize sets the DDR burst size, in bytes, as shown\n\nby the following table."]
    #[inline(always)]
    pub fn burstsize(&self) -> BurstsizeR {
        BurstsizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Register MwrSize sets LPDDR4 data width, which is used for\n\nmasked-write split\n\ncontrol. The field must be set to non-zero for LPDDR3 memories."]
    #[inline(always)]
    pub fn mwrsize(&self) -> MwrsizeR {
        MwrsizeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:15 - When bit n of register field ForceOrder is set to 1, DRAM commands\n\nare executed in the order they arrive at scheduler port n.\n\nWhen field bits are set to 1, and BypassFiltering is also set to 1,\n\ncommand execution order is guaranteed for the corresponding\n\nscheduler port."]
    #[inline(always)]
    pub fn forceorder(&self) -> ForceorderR {
        ForceorderR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ForceOrderState"]
    #[inline(always)]
    pub fn forceorderstate(&self) -> ForceorderstateR {
        ForceorderstateR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When set to one, pages are automatically closed after each access,\n\nwhen set to zero, pages are left opened until an access in a different\n\npage occurs"]
    #[inline(always)]
    #[must_use]
    pub fn autoprecharge(&mut self) -> AutoprechargeW<DdrModeSpec> {
        AutoprechargeW::new(self, 0)
    }
    #[doc = "Bit 1 - When register field BypassFiltering is set to 1, arbiter filters are\n\nbypassed and timing register outputs are internally set to an idle\n\nvalue.The field can be useful during DRAM initialization, when\n\ntraining or calibration sequences are performed, and scheduler\n\narbitration is not needed.\n\nWhen the field is set to 0, scheduler arbitration is fully functional,\n\nthis is the functional usage mode.\n\nNOTE: When the field is set to 1, the final arbitration level continues\n\nto elect transactions among those presented to the arbiter. Set field\n\nForceOrder to ensure that transactions are executed in order, for\n\ninstance during DRAM initialization."]
    #[inline(always)]
    #[must_use]
    pub fn bypassfiltering(&mut self) -> BypassfilteringW<DdrModeSpec> {
        BypassfilteringW::new(self, 1)
    }
    #[doc = "Bit 2 - Register field FawBank indicates the number of banks of a given\n\ndevice involved in the FAW period during which four banks can be\n\nactive.\n\nIt must be set to 0 for 2-bank memories, and 1 for memories with\n\nfour banks or more."]
    #[inline(always)]
    #[must_use]
    pub fn fawbank(&mut self) -> FawbankW<DdrModeSpec> {
        FawbankW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Register field BurstSize sets the DDR burst size, in bytes, as shown\n\nby the following table."]
    #[inline(always)]
    #[must_use]
    pub fn burstsize(&mut self) -> BurstsizeW<DdrModeSpec> {
        BurstsizeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Register MwrSize sets LPDDR4 data width, which is used for\n\nmasked-write split\n\ncontrol. The field must be set to non-zero for LPDDR3 memories."]
    #[inline(always)]
    #[must_use]
    pub fn mwrsize(&mut self) -> MwrsizeW<DdrModeSpec> {
        MwrsizeW::new(self, 5)
    }
    #[doc = "Bits 8:15 - When bit n of register field ForceOrder is set to 1, DRAM commands\n\nare executed in the order they arrive at scheduler port n.\n\nWhen field bits are set to 1, and BypassFiltering is also set to 1,\n\ncommand execution order is guaranteed for the corresponding\n\nscheduler port."]
    #[inline(always)]
    #[must_use]
    pub fn forceorder(&mut self) -> ForceorderW<DdrModeSpec> {
        ForceorderW::new(self, 8)
    }
}
#[doc = "ddr mode definition.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrModeSpec;
impl crate::RegisterSpec for DdrModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_mode::R`](R) reader structure"]
impl crate::Readable for DdrModeSpec {}
#[doc = "`write(|w| ..)` method takes [`ddr_mode::W`](W) writer structure"]
impl crate::Writable for DdrModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DdrMode to value 0x4c"]
impl crate::Resettable for DdrModeSpec {
    const RESET_VALUE: u32 = 0x4c;
}
