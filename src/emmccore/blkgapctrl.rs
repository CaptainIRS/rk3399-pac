#[doc = "Register `BLKGAPCTRL` reader"]
pub type R = crate::R<BlkgapctrlSpec>;
#[doc = "Register `BLKGAPCTRL` writer"]
pub type W = crate::W<BlkgapctrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopatblkgapreq {
    #[doc = "1: Stop"]
    B1 = 1,
    #[doc = "0: Transfer This bit is used to stop executing a transaction at the next block gap for non-DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case ofwrite transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit (DAT) in the Present State register."]
    B0 = 0,
}
impl From<Stopatblkgapreq> for bool {
    #[inline(always)]
    fn from(variant: Stopatblkgapreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPATBLKGAPREQ` reader - "]
pub type StopatblkgapreqR = crate::BitReader<Stopatblkgapreq>;
impl StopatblkgapreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopatblkgapreq {
        match self.bits {
            true => Stopatblkgapreq::B1,
            false => Stopatblkgapreq::B0,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stopatblkgapreq::B1
    }
    #[doc = "Transfer This bit is used to stop executing a transaction at the next block gap for non-DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case ofwrite transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit (DAT) in the Present State register."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stopatblkgapreq::B0
    }
}
#[doc = "Field `STOPATBLKGAPREQ` writer - "]
pub type StopatblkgapreqW<'a, REG> = crate::BitWriter<'a, REG, Stopatblkgapreq>;
impl<'a, REG> StopatblkgapreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stopatblkgapreq::B1)
    }
    #[doc = "Transfer This bit is used to stop executing a transaction at the next block gap for non-DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case ofwrite transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit (DAT) in the Present State register."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stopatblkgapreq::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Continuerequest {
    #[doc = "1: Restart"]
    B1 = 1,
    #[doc = "0: Ignored This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The HC automatically clears this bit in either of the following cases: a. In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. b. In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored."]
    B0 = 0,
}
impl From<Continuerequest> for bool {
    #[inline(always)]
    fn from(variant: Continuerequest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTINUEREQUEST` reader - "]
pub type ContinuerequestR = crate::BitReader<Continuerequest>;
impl ContinuerequestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Continuerequest {
        match self.bits {
            true => Continuerequest::B1,
            false => Continuerequest::B0,
        }
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Continuerequest::B1
    }
    #[doc = "Ignored This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The HC automatically clears this bit in either of the following cases: a. In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. b. In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Continuerequest::B0
    }
}
#[doc = "Field `CONTINUEREQUEST` writer - "]
pub type ContinuerequestW<'a, REG> = crate::BitWriter<'a, REG, Continuerequest>;
impl<'a, REG> ContinuerequestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Restart"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Continuerequest::B1)
    }
    #[doc = "Ignored This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The HC automatically clears this bit in either of the following cases: a. In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. b. In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Continuerequest::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readwaitcontrol {
    #[doc = "1: Enable Read Wait Control"]
    B1 = 1,
    #[doc = "0: Disable Read Wait Control The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported"]
    B0 = 0,
}
impl From<Readwaitcontrol> for bool {
    #[inline(always)]
    fn from(variant: Readwaitcontrol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READWAITCONTROL` reader - "]
pub type ReadwaitcontrolR = crate::BitReader<Readwaitcontrol>;
impl ReadwaitcontrolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readwaitcontrol {
        match self.bits {
            true => Readwaitcontrol::B1,
            false => Readwaitcontrol::B0,
        }
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Readwaitcontrol::B1
    }
    #[doc = "Disable Read Wait Control The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Readwaitcontrol::B0
    }
}
#[doc = "Field `READWAITCONTROL` writer - "]
pub type ReadwaitcontrolW<'a, REG> = crate::BitWriter<'a, REG, Readwaitcontrol>;
impl<'a, REG> ReadwaitcontrolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Readwaitcontrol::B1)
    }
    #[doc = "Disable Read Wait Control The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Readwaitcontrol::B0)
    }
}
#[doc = "Field `INTATBLKGAP` reader - Interrupt At Block Gap.\n\nThis bit is valid only in 4-bit mode of the SDIO card and selects a\n\nsample point in the interrupt cycle. Setting to 1 enables interrupt\n\ndetection at the block gap for a multiple block transfer. If the SD\n\ncard cannot signal an interrupt during a multiple block transfer,\n\nthis bit should be set to 0. When the HD detects an SD card\n\ninsertion, it shall set this bit according to the CCCR of the SDIO\n\ncard."]
pub type IntatblkgapR = crate::BitReader;
#[doc = "Field `INTATBLKGAP` writer - Interrupt At Block Gap.\n\nThis bit is valid only in 4-bit mode of the SDIO card and selects a\n\nsample point in the interrupt cycle. Setting to 1 enables interrupt\n\ndetection at the block gap for a multiple block transfer. If the SD\n\ncard cannot signal an interrupt during a multiple block transfer,\n\nthis bit should be set to 0. When the HD detects an SD card\n\ninsertion, it shall set this bit according to the CCCR of the SDIO\n\ncard."]
pub type IntatblkgapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SPI mode enable bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spimode {
    #[doc = "1: SPI mode"]
    B1 = 1,
    #[doc = "0: SD mode"]
    B0 = 0,
}
impl From<Spimode> for bool {
    #[inline(always)]
    fn from(variant: Spimode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIMODE` reader - SPI mode enable bit."]
pub type SpimodeR = crate::BitReader<Spimode>;
impl SpimodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spimode {
        match self.bits {
            true => Spimode::B1,
            false => Spimode::B0,
        }
    }
    #[doc = "SPI mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spimode::B1
    }
    #[doc = "SD mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spimode::B0
    }
}
#[doc = "Field `SPIMODE` writer - SPI mode enable bit."]
pub type SpimodeW<'a, REG> = crate::BitWriter<'a, REG, Spimode>;
impl<'a, REG> SpimodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::B1)
    }
    #[doc = "SD mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::B0)
    }
}
#[doc = "To start boot code access.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Booten {
    #[doc = "1: To start boot code access"]
    B1 = 1,
    #[doc = "0: To stop boot code access"]
    B0 = 0,
}
impl From<Booten> for bool {
    #[inline(always)]
    fn from(variant: Booten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTEN` reader - To start boot code access."]
pub type BootenR = crate::BitReader<Booten>;
impl BootenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Booten {
        match self.bits {
            true => Booten::B1,
            false => Booten::B0,
        }
    }
    #[doc = "To start boot code access"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Booten::B1
    }
    #[doc = "To stop boot code access"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Booten::B0
    }
}
#[doc = "Field `BOOTEN` writer - To start boot code access."]
pub type BootenW<'a, REG> = crate::BitWriter<'a, REG, Booten>;
impl<'a, REG> BootenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "To start boot code access"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Booten::B1)
    }
    #[doc = "To stop boot code access"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Booten::B0)
    }
}
#[doc = "To start boot code access in alternative mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Altbooten {
    #[doc = "1: To start alternate boot mode access"]
    B1 = 1,
    #[doc = "0: To stop alternate boot mode access"]
    B0 = 0,
}
impl From<Altbooten> for bool {
    #[inline(always)]
    fn from(variant: Altbooten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALTBOOTEN` reader - To start boot code access in alternative mode."]
pub type AltbootenR = crate::BitReader<Altbooten>;
impl AltbootenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Altbooten {
        match self.bits {
            true => Altbooten::B1,
            false => Altbooten::B0,
        }
    }
    #[doc = "To start alternate boot mode access"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Altbooten::B1
    }
    #[doc = "To stop alternate boot mode access"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Altbooten::B0
    }
}
#[doc = "Field `ALTBOOTEN` writer - To start boot code access in alternative mode."]
pub type AltbootenW<'a, REG> = crate::BitWriter<'a, REG, Altbooten>;
impl<'a, REG> AltbootenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "To start alternate boot mode access"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Altbooten::B1)
    }
    #[doc = "To stop alternate boot mode access"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Altbooten::B0)
    }
}
#[doc = "To check for the boot acknowledge in boot operation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootackchk {
    #[doc = "1: wait for boot ack from eMMC card"]
    B1 = 1,
    #[doc = "0: Will not wait for boot ack from eMMC card"]
    B0 = 0,
}
impl From<Bootackchk> for bool {
    #[inline(always)]
    fn from(variant: Bootackchk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTACKCHK` reader - To check for the boot acknowledge in boot operation."]
pub type BootackchkR = crate::BitReader<Bootackchk>;
impl BootackchkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootackchk {
        match self.bits {
            true => Bootackchk::B1,
            false => Bootackchk::B0,
        }
    }
    #[doc = "wait for boot ack from eMMC card"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bootackchk::B1
    }
    #[doc = "Will not wait for boot ack from eMMC card"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bootackchk::B0
    }
}
#[doc = "Field `BOOTACKCHK` writer - To check for the boot acknowledge in boot operation."]
pub type BootackchkW<'a, REG> = crate::BitWriter<'a, REG, Bootackchk>;
impl<'a, REG> BootackchkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wait for boot ack from eMMC card"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackchk::B1)
    }
    #[doc = "Will not wait for boot ack from eMMC card"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackchk::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stopatblkgapreq(&self) -> StopatblkgapreqR {
        StopatblkgapreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn continuerequest(&self) -> ContinuerequestR {
        ContinuerequestR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn readwaitcontrol(&self) -> ReadwaitcontrolR {
        ReadwaitcontrolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap.\n\nThis bit is valid only in 4-bit mode of the SDIO card and selects a\n\nsample point in the interrupt cycle. Setting to 1 enables interrupt\n\ndetection at the block gap for a multiple block transfer. If the SD\n\ncard cannot signal an interrupt during a multiple block transfer,\n\nthis bit should be set to 0. When the HD detects an SD card\n\ninsertion, it shall set this bit according to the CCCR of the SDIO\n\ncard."]
    #[inline(always)]
    pub fn intatblkgap(&self) -> IntatblkgapR {
        IntatblkgapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI mode enable bit."]
    #[inline(always)]
    pub fn spimode(&self) -> SpimodeR {
        SpimodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - To start boot code access."]
    #[inline(always)]
    pub fn booten(&self) -> BootenR {
        BootenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - To start boot code access in alternative mode."]
    #[inline(always)]
    pub fn altbooten(&self) -> AltbootenR {
        AltbootenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - To check for the boot acknowledge in boot operation."]
    #[inline(always)]
    pub fn bootackchk(&self) -> BootackchkR {
        BootackchkR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn stopatblkgapreq(&mut self) -> StopatblkgapreqW<BlkgapctrlSpec> {
        StopatblkgapreqW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn continuerequest(&mut self) -> ContinuerequestW<BlkgapctrlSpec> {
        ContinuerequestW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn readwaitcontrol(&mut self) -> ReadwaitcontrolW<BlkgapctrlSpec> {
        ReadwaitcontrolW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap.\n\nThis bit is valid only in 4-bit mode of the SDIO card and selects a\n\nsample point in the interrupt cycle. Setting to 1 enables interrupt\n\ndetection at the block gap for a multiple block transfer. If the SD\n\ncard cannot signal an interrupt during a multiple block transfer,\n\nthis bit should be set to 0. When the HD detects an SD card\n\ninsertion, it shall set this bit according to the CCCR of the SDIO\n\ncard."]
    #[inline(always)]
    #[must_use]
    pub fn intatblkgap(&mut self) -> IntatblkgapW<BlkgapctrlSpec> {
        IntatblkgapW::new(self, 3)
    }
    #[doc = "Bit 4 - SPI mode enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn spimode(&mut self) -> SpimodeW<BlkgapctrlSpec> {
        SpimodeW::new(self, 4)
    }
    #[doc = "Bit 5 - To start boot code access."]
    #[inline(always)]
    #[must_use]
    pub fn booten(&mut self) -> BootenW<BlkgapctrlSpec> {
        BootenW::new(self, 5)
    }
    #[doc = "Bit 6 - To start boot code access in alternative mode."]
    #[inline(always)]
    #[must_use]
    pub fn altbooten(&mut self) -> AltbootenW<BlkgapctrlSpec> {
        AltbootenW::new(self, 6)
    }
    #[doc = "Bit 7 - To check for the boot acknowledge in boot operation."]
    #[inline(always)]
    #[must_use]
    pub fn bootackchk(&mut self) -> BootackchkW<BlkgapctrlSpec> {
        BootackchkW::new(self, 7)
    }
}
#[doc = "Block gap control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blkgapctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blkgapctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkgapctrlSpec;
impl crate::RegisterSpec for BlkgapctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`blkgapctrl::R`](R) reader structure"]
impl crate::Readable for BlkgapctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`blkgapctrl::W`](W) writer structure"]
impl crate::Writable for BlkgapctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BLKGAPCTRL to value 0"]
impl crate::Resettable for BlkgapctrlSpec {
    const RESET_VALUE: u8 = 0;
}
