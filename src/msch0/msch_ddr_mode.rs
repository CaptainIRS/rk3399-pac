#[doc = "Register `MSCH_DdrMode` reader"]
pub type R = crate::R<MschDdrModeSpec>;
#[doc = "Register `MSCH_DdrMode` writer"]
pub type W = crate::W<MschDdrModeSpec>;
#[doc = "Field `AUTOPRECHARGE` reader - When set to one, pages are automatically closed after each access, when set to zero, pages are left opened until an access in a different page occurs"]
pub type AutoprechargeR = crate::BitReader;
#[doc = "Field `AUTOPRECHARGE` writer - When set to one, pages are automatically closed after each access, when set to zero, pages are left opened until an access in a different page occurs"]
pub type AutoprechargeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASSFILTERING` reader - When register field BypassFiltering is set to 1, arbiter filters are bypassed and timing register outputs are internally set to an idle value.The field can be useful during DRAM initialization, when training or calibration sequences are performed, and scheduler arbitration is not needed. When the field is set to 0, scheduler arbitration is fully functional, this is the functional usage mode. NOTE: When the field is set to 1, the final arbitration level continues to elect transactions among those presented to the arbiter. Set field ForceOrder to ensure that transactions are executed in order, for instance during DRAM initialization."]
pub type BypassfilteringR = crate::BitReader;
#[doc = "Field `BYPASSFILTERING` writer - When register field BypassFiltering is set to 1, arbiter filters are bypassed and timing register outputs are internally set to an idle value.The field can be useful during DRAM initialization, when training or calibration sequences are performed, and scheduler arbitration is not needed. When the field is set to 0, scheduler arbitration is fully functional, this is the functional usage mode. NOTE: When the field is set to 1, the final arbitration level continues to elect transactions among those presented to the arbiter. Set field ForceOrder to ensure that transactions are executed in order, for instance during DRAM initialization."]
pub type BypassfilteringW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAWBANK` reader - Register field FawBank indicates the number of banks of a given device involved in the FAW period during which four banks can be active. It must be set to 0 for 2-bank memories, and 1 for memories with four banks or more."]
pub type FawbankR = crate::BitReader;
#[doc = "Field `FAWBANK` writer - Register field FawBank indicates the number of banks of a given device involved in the FAW period during which four banks can be active. It must be set to 0 for 2-bank memories, and 1 for memories with four banks or more."]
pub type FawbankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Register field BurstSize sets the DDR burst size, in bytes, as shown by the following table.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Burstsize {
    #[doc = "0: 128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
    B00 = 0,
    #[doc = "1: 128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
    B01 = 1,
    #[doc = "2: 128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
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
#[doc = "Field `BURSTSIZE` reader - Register field BurstSize sets the DDR burst size, in bytes, as shown by the following table."]
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
    #[doc = "128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Burstsize::B00
    }
    #[doc = "128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Burstsize::B01
    }
    #[doc = "128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
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
#[doc = "Field `BURSTSIZE` writer - Register field BurstSize sets the DDR burst size, in bytes, as shown by the following table."]
pub type BurstsizeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Burstsize>;
impl<'a, REG> BurstsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsize::B00)
    }
    #[doc = "128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Burstsize::B01)
    }
    #[doc = "128 NOTE: For LPDDR4 memories, the field must be set to the number of bytes required by BL16 transactions."]
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
#[doc = "Register MwrSize sets LPDDR4 data width, which is used for masked-write split control. The field must be set to non-zero for LPDDR3 memories.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mwrsize {
    #[doc = "0: LPDDR4, 32 bits."]
    B00 = 0,
    #[doc = "1: LPDDR4, 32 bits."]
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
#[doc = "Field `MWRSIZE` reader - Register MwrSize sets LPDDR4 data width, which is used for masked-write split control. The field must be set to non-zero for LPDDR3 memories."]
pub type MwrsizeR = crate::FieldReader<Mwrsize>;
impl MwrsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mwrsize {
        match self.bits {
            0 => Mwrsize::B00,
            1 => Mwrsize::B01,
            2 => Mwrsize::B10,
            3 => Mwrsize::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "LPDDR4, 32 bits."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Mwrsize::B00
    }
    #[doc = "LPDDR4, 32 bits."]
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
#[doc = "Field `MWRSIZE` writer - Register MwrSize sets LPDDR4 data width, which is used for masked-write split control. The field must be set to non-zero for LPDDR3 memories."]
pub type MwrsizeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Mwrsize>;
impl<'a, REG> MwrsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPDDR4, 32 bits."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Mwrsize::B00)
    }
    #[doc = "LPDDR4, 32 bits."]
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
#[doc = "Field `FORCEORDER` reader - When bit n of register field ForceOrder is set to 1, DRAM commands are executed in the order they arrive at scheduler port n. When field bits are set to 1, and BypassFiltering is also set to 1, command execution order is guaranteed for the corresponding scheduler port."]
pub type ForceorderR = crate::FieldReader;
#[doc = "Field `FORCEORDER` writer - When bit n of register field ForceOrder is set to 1, DRAM commands are executed in the order they arrive at scheduler port n. When field bits are set to 1, and BypassFiltering is also set to 1, command execution order is guaranteed for the corresponding scheduler port."]
pub type ForceorderW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FORCEORDERSTATE` reader - ForceOrderState"]
pub type ForceorderstateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - When set to one, pages are automatically closed after each access, when set to zero, pages are left opened until an access in a different page occurs"]
    #[inline(always)]
    pub fn autoprecharge(&self) -> AutoprechargeR {
        AutoprechargeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When register field BypassFiltering is set to 1, arbiter filters are bypassed and timing register outputs are internally set to an idle value.The field can be useful during DRAM initialization, when training or calibration sequences are performed, and scheduler arbitration is not needed. When the field is set to 0, scheduler arbitration is fully functional, this is the functional usage mode. NOTE: When the field is set to 1, the final arbitration level continues to elect transactions among those presented to the arbiter. Set field ForceOrder to ensure that transactions are executed in order, for instance during DRAM initialization."]
    #[inline(always)]
    pub fn bypassfiltering(&self) -> BypassfilteringR {
        BypassfilteringR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register field FawBank indicates the number of banks of a given device involved in the FAW period during which four banks can be active. It must be set to 0 for 2-bank memories, and 1 for memories with four banks or more."]
    #[inline(always)]
    pub fn fawbank(&self) -> FawbankR {
        FawbankR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Register field BurstSize sets the DDR burst size, in bytes, as shown by the following table."]
    #[inline(always)]
    pub fn burstsize(&self) -> BurstsizeR {
        BurstsizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Register MwrSize sets LPDDR4 data width, which is used for masked-write split control. The field must be set to non-zero for LPDDR3 memories."]
    #[inline(always)]
    pub fn mwrsize(&self) -> MwrsizeR {
        MwrsizeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:15 - When bit n of register field ForceOrder is set to 1, DRAM commands are executed in the order they arrive at scheduler port n. When field bits are set to 1, and BypassFiltering is also set to 1, command execution order is guaranteed for the corresponding scheduler port."]
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
    #[doc = "Bit 0 - When set to one, pages are automatically closed after each access, when set to zero, pages are left opened until an access in a different page occurs"]
    #[inline(always)]
    #[must_use]
    pub fn autoprecharge(&mut self) -> AutoprechargeW<MschDdrModeSpec> {
        AutoprechargeW::new(self, 0)
    }
    #[doc = "Bit 1 - When register field BypassFiltering is set to 1, arbiter filters are bypassed and timing register outputs are internally set to an idle value.The field can be useful during DRAM initialization, when training or calibration sequences are performed, and scheduler arbitration is not needed. When the field is set to 0, scheduler arbitration is fully functional, this is the functional usage mode. NOTE: When the field is set to 1, the final arbitration level continues to elect transactions among those presented to the arbiter. Set field ForceOrder to ensure that transactions are executed in order, for instance during DRAM initialization."]
    #[inline(always)]
    #[must_use]
    pub fn bypassfiltering(&mut self) -> BypassfilteringW<MschDdrModeSpec> {
        BypassfilteringW::new(self, 1)
    }
    #[doc = "Bit 2 - Register field FawBank indicates the number of banks of a given device involved in the FAW period during which four banks can be active. It must be set to 0 for 2-bank memories, and 1 for memories with four banks or more."]
    #[inline(always)]
    #[must_use]
    pub fn fawbank(&mut self) -> FawbankW<MschDdrModeSpec> {
        FawbankW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Register field BurstSize sets the DDR burst size, in bytes, as shown by the following table."]
    #[inline(always)]
    #[must_use]
    pub fn burstsize(&mut self) -> BurstsizeW<MschDdrModeSpec> {
        BurstsizeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Register MwrSize sets LPDDR4 data width, which is used for masked-write split control. The field must be set to non-zero for LPDDR3 memories."]
    #[inline(always)]
    #[must_use]
    pub fn mwrsize(&mut self) -> MwrsizeW<MschDdrModeSpec> {
        MwrsizeW::new(self, 5)
    }
    #[doc = "Bits 8:15 - When bit n of register field ForceOrder is set to 1, DRAM commands are executed in the order they arrive at scheduler port n. When field bits are set to 1, and BypassFiltering is also set to 1, command execution order is guaranteed for the corresponding scheduler port."]
    #[inline(always)]
    #[must_use]
    pub fn forceorder(&mut self) -> ForceorderW<MschDdrModeSpec> {
        ForceorderW::new(self, 8)
    }
}
#[doc = "ddr mode definition.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_ddr_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_ddr_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MschDdrModeSpec;
impl crate::RegisterSpec for MschDdrModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msch_ddr_mode::R`](R) reader structure"]
impl crate::Readable for MschDdrModeSpec {}
#[doc = "`write(|w| ..)` method takes [`msch_ddr_mode::W`](W) writer structure"]
impl crate::Writable for MschDdrModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSCH_DdrMode to value 0x4c"]
impl crate::Resettable for MschDdrModeSpec {
    const RESET_VALUE: u32 = 0x4c;
}
