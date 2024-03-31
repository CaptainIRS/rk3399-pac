#[doc = "Register `TRANSMOD` reader"]
pub type R = crate::R<TransmodSpec>;
#[doc = "Register `TRANSMOD` writer"]
pub type W = crate::W<TransmodSpec>;
#[doc = "DMA can be enabled only if DMA Support bit in the Capabilities\n\nregister is set. If this bit is set to 1, a DMA operation shall begin\n\nwhen the HD writes to the upper byte of Command register\n\n(00Fh).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaenable {
    #[doc = "0: Disable"]
    B0 = 0,
    #[doc = "1: Enable"]
    B1 = 1,
}
impl From<Dmaenable> for bool {
    #[inline(always)]
    fn from(variant: Dmaenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAENABLE` reader - DMA can be enabled only if DMA Support bit in the Capabilities\n\nregister is set. If this bit is set to 1, a DMA operation shall begin\n\nwhen the HD writes to the upper byte of Command register\n\n(00Fh)."]
pub type DmaenableR = crate::BitReader<Dmaenable>;
impl DmaenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaenable {
        match self.bits {
            false => Dmaenable::B0,
            true => Dmaenable::B1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dmaenable::B0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dmaenable::B1
    }
}
#[doc = "Field `DMAENABLE` writer - DMA can be enabled only if DMA Support bit in the Capabilities\n\nregister is set. If this bit is set to 1, a DMA operation shall begin\n\nwhen the HD writes to the upper byte of Command register\n\n(00Fh)."]
pub type DmaenableW<'a, REG> = crate::BitWriter<'a, REG, Dmaenable>;
impl<'a, REG> DmaenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaenable::B0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaenable::B1)
    }
}
#[doc = "This bit is used to enable the Block count register, which is only\n\nrelevant for multiple block transfers. When this bit is 0, the Block\n\nCount register is disabled, which is useful in executing an infinite\n\ntransfer.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockcountenable {
    #[doc = "0: Disable"]
    B0 = 0,
    #[doc = "1: Enable"]
    B1 = 1,
}
impl From<Blockcountenable> for bool {
    #[inline(always)]
    fn from(variant: Blockcountenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKCOUNTENABLE` reader - This bit is used to enable the Block count register, which is only\n\nrelevant for multiple block transfers. When this bit is 0, the Block\n\nCount register is disabled, which is useful in executing an infinite\n\ntransfer."]
pub type BlockcountenableR = crate::BitReader<Blockcountenable>;
impl BlockcountenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockcountenable {
        match self.bits {
            false => Blockcountenable::B0,
            true => Blockcountenable::B1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Blockcountenable::B0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Blockcountenable::B1
    }
}
#[doc = "Field `BLOCKCOUNTENABLE` writer - This bit is used to enable the Block count register, which is only\n\nrelevant for multiple block transfers. When this bit is 0, the Block\n\nCount register is disabled, which is useful in executing an infinite\n\ntransfer."]
pub type BlockcountenableW<'a, REG> = crate::BitWriter<'a, REG, Blockcountenable>;
impl<'a, REG> BlockcountenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Blockcountenable::B0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Blockcountenable::B1)
    }
}
#[doc = "This field determines use of auto command functions\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Autocmdenable {
    #[doc = "0: Auto Command Disabled"]
    D0 = 0,
    #[doc = "1: Auto CMD12 Enable"]
    D1 = 1,
    #[doc = "2: Auto CMD23 Enable"]
    D2 = 2,
    #[doc = "3: Reserved There are two methods to stop Multiple-block read and write operation. (1) Auto CMD12 Enable Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transferis completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. (2) Auto CMD23 Enable When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command Register The following conditions are required to use the Auto CMD23. a. Auto CMD23 Supported (Host Controller Version is 3.00 or later) b. A memory card that supports CMD23 (SCR\\[33\\]=1) c. If DMA is used, it shall be ADMA d. Only when CMD18 or CMD25 is issued By writing the Command register, the Host Controller issues a CMD23 first and then issues a command specified by the Command Index in Command register 32-bit block count value for CMD23 is set to SDMA System Address / Argument 2 register"]
    D3 = 3,
}
impl From<Autocmdenable> for u8 {
    #[inline(always)]
    fn from(variant: Autocmdenable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Autocmdenable {
    type Ux = u8;
}
#[doc = "Field `AUTOCMDENABLE` reader - This field determines use of auto command functions"]
pub type AutocmdenableR = crate::FieldReader<Autocmdenable>;
impl AutocmdenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autocmdenable {
        match self.bits {
            0 => Autocmdenable::D0,
            1 => Autocmdenable::D1,
            2 => Autocmdenable::D2,
            3 => Autocmdenable::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Autocmdenable::D0
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Autocmdenable::D1
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Autocmdenable::D2
    }
    #[doc = "Reserved There are two methods to stop Multiple-block read and write operation. (1) Auto CMD12 Enable Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transferis completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. (2) Auto CMD23 Enable When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command Register The following conditions are required to use the Auto CMD23. a. Auto CMD23 Supported (Host Controller Version is 3.00 or later) b. A memory card that supports CMD23 (SCR\\[33\\]=1) c. If DMA is used, it shall be ADMA d. Only when CMD18 or CMD25 is issued By writing the Command register, the Host Controller issues a CMD23 first and then issues a command specified by the Command Index in Command register 32-bit block count value for CMD23 is set to SDMA System Address / Argument 2 register"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Autocmdenable::D3
    }
}
#[doc = "Field `AUTOCMDENABLE` writer - This field determines use of auto command functions"]
pub type AutocmdenableW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Autocmdenable>;
impl<'a, REG> AutocmdenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmdenable::D0)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmdenable::D1)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmdenable::D2)
    }
    #[doc = "Reserved There are two methods to stop Multiple-block read and write operation. (1) Auto CMD12 Enable Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transferis completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. (2) Auto CMD23 Enable When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command Register The following conditions are required to use the Auto CMD23. a. Auto CMD23 Supported (Host Controller Version is 3.00 or later) b. A memory card that supports CMD23 (SCR\\[33\\]=1) c. If DMA is used, it shall be ADMA d. Only when CMD18 or CMD25 is issued By writing the Command register, the Host Controller issues a CMD23 first and then issues a command specified by the Command Index in Command register 32-bit block count value for CMD23 is set to SDMA System Address / Argument 2 register"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmdenable::D3)
    }
}
#[doc = "This bit defines the direction of data transfers.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datatransferdirectionselect {
    #[doc = "0: Write (Host to Card)"]
    B0 = 0,
    #[doc = "1: Read (Card to Host)"]
    B1 = 1,
}
impl From<Datatransferdirectionselect> for bool {
    #[inline(always)]
    fn from(variant: Datatransferdirectionselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATATRANSFERDIRECTIONSELECT` reader - This bit defines the direction of data transfers."]
pub type DatatransferdirectionselectR = crate::BitReader<Datatransferdirectionselect>;
impl DatatransferdirectionselectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datatransferdirectionselect {
        match self.bits {
            false => Datatransferdirectionselect::B0,
            true => Datatransferdirectionselect::B1,
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datatransferdirectionselect::B0
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datatransferdirectionselect::B1
    }
}
#[doc = "Field `DATATRANSFERDIRECTIONSELECT` writer - This bit defines the direction of data transfers."]
pub type DatatransferdirectionselectW<'a, REG> =
    crate::BitWriter<'a, REG, Datatransferdirectionselect>;
impl<'a, REG> DatatransferdirectionselectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datatransferdirectionselect::B0)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datatransferdirectionselect::B1)
    }
}
#[doc = "This bit enables multiple block data transfers.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Multiblockselect {
    #[doc = "0: Single Block"]
    B0 = 0,
    #[doc = "1: Multiple Block"]
    B1 = 1,
}
impl From<Multiblockselect> for bool {
    #[inline(always)]
    fn from(variant: Multiblockselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULTIBLOCKSELECT` reader - This bit enables multiple block data transfers."]
pub type MultiblockselectR = crate::BitReader<Multiblockselect>;
impl MultiblockselectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Multiblockselect {
        match self.bits {
            false => Multiblockselect::B0,
            true => Multiblockselect::B1,
        }
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Multiblockselect::B0
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Multiblockselect::B1
    }
}
#[doc = "Field `MULTIBLOCKSELECT` writer - This bit enables multiple block data transfers."]
pub type MultiblockselectW<'a, REG> = crate::BitWriter<'a, REG, Multiblockselect>;
impl<'a, REG> MultiblockselectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Multiblockselect::B0)
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Multiblockselect::B1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA can be enabled only if DMA Support bit in the Capabilities\n\nregister is set. If this bit is set to 1, a DMA operation shall begin\n\nwhen the HD writes to the upper byte of Command register\n\n(00Fh)."]
    #[inline(always)]
    pub fn dmaenable(&self) -> DmaenableR {
        DmaenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to enable the Block count register, which is only\n\nrelevant for multiple block transfers. When this bit is 0, the Block\n\nCount register is disabled, which is useful in executing an infinite\n\ntransfer."]
    #[inline(always)]
    pub fn blockcountenable(&self) -> BlockcountenableR {
        BlockcountenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This field determines use of auto command functions"]
    #[inline(always)]
    pub fn autocmdenable(&self) -> AutocmdenableR {
        AutocmdenableR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - This bit defines the direction of data transfers."]
    #[inline(always)]
    pub fn datatransferdirectionselect(&self) -> DatatransferdirectionselectR {
        DatatransferdirectionselectR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit enables multiple block data transfers."]
    #[inline(always)]
    pub fn multiblockselect(&self) -> MultiblockselectR {
        MultiblockselectR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA can be enabled only if DMA Support bit in the Capabilities\n\nregister is set. If this bit is set to 1, a DMA operation shall begin\n\nwhen the HD writes to the upper byte of Command register\n\n(00Fh)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaenable(&mut self) -> DmaenableW<TransmodSpec> {
        DmaenableW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is used to enable the Block count register, which is only\n\nrelevant for multiple block transfers. When this bit is 0, the Block\n\nCount register is disabled, which is useful in executing an infinite\n\ntransfer."]
    #[inline(always)]
    #[must_use]
    pub fn blockcountenable(&mut self) -> BlockcountenableW<TransmodSpec> {
        BlockcountenableW::new(self, 1)
    }
    #[doc = "Bits 2:3 - This field determines use of auto command functions"]
    #[inline(always)]
    #[must_use]
    pub fn autocmdenable(&mut self) -> AutocmdenableW<TransmodSpec> {
        AutocmdenableW::new(self, 2)
    }
    #[doc = "Bit 4 - This bit defines the direction of data transfers."]
    #[inline(always)]
    #[must_use]
    pub fn datatransferdirectionselect(&mut self) -> DatatransferdirectionselectW<TransmodSpec> {
        DatatransferdirectionselectW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit enables multiple block data transfers."]
    #[inline(always)]
    #[must_use]
    pub fn multiblockselect(&mut self) -> MultiblockselectW<TransmodSpec> {
        MultiblockselectW::new(self, 5)
    }
}
#[doc = "Transfer mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransmodSpec;
impl crate::RegisterSpec for TransmodSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`transmod::R`](R) reader structure"]
impl crate::Readable for TransmodSpec {}
#[doc = "`write(|w| ..)` method takes [`transmod::W`](W) writer structure"]
impl crate::Writable for TransmodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TRANSMOD to value 0"]
impl crate::Resettable for TransmodSpec {
    const RESET_VALUE: u16 = 0;
}
